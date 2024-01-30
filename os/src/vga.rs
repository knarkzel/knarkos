use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::port::Port;

use crate::serial_println;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row: 0,
        column: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

pub fn init() {
    let mut vga = WRITER.lock();
    vga.clear_screen();
    vga.write_prompt();
}

// Abstractions for color
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

// Setup buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const PROMPT_LENGTH: usize = 10;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row: usize,
    column: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_prompt(&mut self) {
        self.color_code = ColorCode::new(Color::Green, Color::Black);
        self.write_string("knarkos ");
        self.color_code = ColorCode::new(Color::LightBlue, Color::Black);
        self.write_string("> ");
        self.color_code = ColorCode::new(Color::White, Color::Black);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => {
                    serial_println!("Got invalid key: {byte}");
                }
            }
        }
        self.move_cursor();
    }

    pub fn delete_character(&mut self) {
        if self.column > PROMPT_LENGTH {
            self.column -= 1;
            self.write_byte_to(b' ', self.row, self.column);
        }
    }

    fn write_byte_to(&mut self, byte: u8, row: usize, column: usize) {
        self.buffer.chars[row][column].write(ScreenChar {
            ascii_character: byte,
            color_code: self.color_code,
        });
    }

    fn write_byte(&mut self, byte: u8) {
        // Write the byte
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.write_byte_to(byte, self.row, self.column);
                self.column += 1;
            }
        }
    }

    fn move_cursor(&self) {
        // Move VGA cursor
        let position = self.row * BUFFER_WIDTH + self.column;
        let mut cursor_control = Port::<u8>::new(0x3D4);
        let mut cursor_register = Port::<u8>::new(0x3D5);
        unsafe {
            cursor_control.write(0x0F);
            cursor_register.write((position & 0xFF) as u8);
            cursor_control.write(0x0E);
            cursor_register.write(((position >> 8) & 0xFF) as u8);
        }
    }

    fn new_line(&mut self) {
        if self.row >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.row += 1;
        }
        self.column = 0;
        self.write_prompt();
    }

    fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
