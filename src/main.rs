#![feature(custom_test_frameworks)]
#![test_runner(martim::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

#[allow(unused_imports)] // not actually unused
use martim::{hlt_loop, println};
use martim::{allocator, context, memory};
use martim::memory::BootInfoFrameAllocator;
use martim::task::{keyboard, Task};
use martim::task::executor::Executor;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    martim::test_panic_handler(info)
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    martim::init();
    init_heap(boot_info);
    context::init();

    #[cfg(not(test))]
        main();

    #[cfg(test)]
        test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

fn init_heap(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
}

fn main() {
    println!("Hello, {}!", "World");
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}