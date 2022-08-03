#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(the_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use the_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!!");

    the_os::init();

    x86_64::instructions::interrupts::int3();

    println!("It did not crash");

    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    the_os::test_panic_handler(info)
}
