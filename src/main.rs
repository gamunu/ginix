#![no_std] // don't link the Rust standard library
#![no_main] // disabled all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(ginix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use ginix::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// This function called on xtest panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ginix::test_panic_handler(info)
}