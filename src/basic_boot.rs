#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle] // i wonder why compilers mangle names lol
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimlpemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}