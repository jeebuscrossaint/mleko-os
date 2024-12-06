
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

//mod vga_buffer; moved to lib.rs
//mod serial; moved to lib.rs
use mleko::test_runner;
use core::panic::PanicInfo;
use mleko::println;


/// This function is called on panic.
#[cfg(not(test))] // new attribute dropped ayyy
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mleko::test_panic_handler(info)
}


// from old vga implementation
// static VGA_BUF: &[u8] = b"Hello, world!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    // old vga implementation
    /*let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in VGA_BUF.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    } */

    // -- slightly newer but now old/outdated vga implementation
    /*
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    */

    println!("Hello World{}", "!");
    //example panic to test panic handler
    //panic!("Some panic message");

    mleko::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    
    println!("we are alive");
    loop {}
}


// DO NOTE THAT IN CARGO AS OF TO MY KNOWLEDGE 2019 AND EVEN NOW IN 2024 
// THERE IS A BUG IN CARGO WHERE THERE ARE "duplicate lang item" ERRORS 
// ON CARGO TEST IN SOME CASES. TO FIX REMOVE/COMMENT OUT THE ' panic = "abort" ' 
// FOR A PROFILE IN THE CARGO.TOML FILE
/* #[cfg(test)] old test runner moved to lib.rs
pub fn test_runner(tests: &[&dyn Testable]) { // new
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
} */

/*#[test_case] // an old test case
fn trivial_assertion() {
    assert_eq!(1, 1);
}
*/

/* old qemu stuff moved to lib.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
*/