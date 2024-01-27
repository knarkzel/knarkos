#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(knarkos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use knarkos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    knarkos::test_panic_handler(info)
}
