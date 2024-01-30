#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(knarkos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use knarkos::{println, repl};
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use knarkos::{memory, allocator};
    use knarkos::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;
    
    // Initialize operating system
    knarkos::init();

    // Setup heap
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    #[cfg(test)]
    test_main();

    // Run repl
    #[cfg(not(test))]
    repl::main();
    
    knarkos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    knarkos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    knarkos::test_panic_handler(info)
}
