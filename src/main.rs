#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(the_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use the_os::{println, print};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!!");

    the_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    the_os::hlt_loop();
    //loop {
        //print!("-");
        //for _ in 0..10000 {}
    //}
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
