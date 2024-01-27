#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(knarkos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use knarkos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize operating system
    knarkos::init();
    println!("Hello, world!");

    #[cfg(test)]
    test_main();

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
