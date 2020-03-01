#![no_std] // don't link the Rust standard library
#![no_main] // disabled all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

/// This function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
