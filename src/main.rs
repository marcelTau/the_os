#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(the_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use the_os::{println, memory, allocator};
use core::panic::PanicInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::Page;
use alloc::boxed::Box;

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!!!");
    the_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(42);

    println!("It did not crash again");

    let page: Page = Page::containing_address(VirtAddr::new(0xb8000));

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)
    };


    #[cfg(test)]
    test_main();

    println!("It did not crash");
    the_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    the_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    the_os::test_panic_handler(info)
}
