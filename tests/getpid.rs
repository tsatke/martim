#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(martim::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use martim::{context, syscall};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use martim::allocator;
    use martim::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    martim::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    context::init();

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    martim::test_panic_handler(info)
}

#[test_case]
fn getpid() {
    // test that (also for repeated calls) the pid is 0 (first process/thread)

    assert_eq!(0, syscall::getpid().unwrap().into());
    assert_eq!(0, syscall::getpid().unwrap().into());
    assert_eq!(0, syscall::getpid().unwrap().into());
}