#![feature(panic_info_message)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("Panic: '{}' at {}", _info.message().unwrap(), _info.location().unwrap());
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    panic!("_start returned");
}

fn main() {
    println!("Hello, {}!", "World");
}
