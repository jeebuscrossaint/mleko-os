#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mlekoczekoladowe::println;
use mlekoczekoladowe::test_runner;

#[no_mangle] // i wonder why compilers mangle names lol
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


//#[test_runner(mlekoczekoladowe::test_runner)]

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}