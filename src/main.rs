#![feature(custom_test_frameworks)]
#![test_runner(martim::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use martim::println;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    martim::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
        main();

    #[cfg(test)]
        test_main();

    panic!("_start returned");
}

fn main() {
    println!("Hello, {}!", "World");
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}