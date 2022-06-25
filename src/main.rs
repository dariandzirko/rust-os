#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
use rust_os::println;

//This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    //panic!("Some panic message");
    
    // this function is the entry point, since the linker looks for a function 
    // named '_start' by default
    loop {}
}

