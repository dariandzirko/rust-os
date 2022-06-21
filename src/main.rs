#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::{panic::PanicInfo, fmt::Write};

//This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");
    
    //panic!("Some panic message");
    
    // this function is the entry point, since the linker looks for a function 
    // named '_start' by default
    loop {}
}
