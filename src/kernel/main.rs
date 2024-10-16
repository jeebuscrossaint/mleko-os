



extern "C" {
    //static startothers: fn();
    #[noreturn]
    static mpmain: fn();
    static kpgdir: *mut pde_t;
    static end: [u8; 0]; // first address after kernel loaded from ELF file
}

#[noreturn]
fn main() {
    uartearlyinit();
    kinit1(end, p2v(4 * 1024 * 1024)); // phys page allocator
    kvmalloc(); // kernel page table

    // Try to use ACPI for machine info, otherwise use BIOS MP tables
    if acpiinit() {
        mpinit();
    }

    lapicinit();
    seginit(); // set up segments

    // Assuming `cpu` is some globally accessible CPU structure, like in C
    // and has an id field, which is usually obtained in a safe way in Rust.
    println!("\ncpu{}: starting xv6\n\n", cpu().id);

    picinit(); // interrupt controller
    ioapicinit(); // another interrupt controller
    consoleinit(); // I/O devices & their interrupts
    uartinit(); // serial port
    pinit(); // process table
    tvinit(); // trap vectors
    binit(); // buffer cache
    fileinit(); // file table
    iinit(); // inode cache
    ideinit(); // disk

    // Initialize the timer for uniprocessor systems
    if !ismp() {
        timerinit();
    }

    startothers(); // start other processors

    // Initialize the second part of memory allocation
    kinit2(p2v(4 * 1024 * 1024), p2v(PHYSTOP));

    userinit(); // first user process
    cpuidinit(); // CPUID initialization

    // Finish setting up this processor in mpmain
    unsafe {
        mpmain(); // `mpmain` is marked as a `#[noreturn]` function in the extern block
    }
}
#[no_mangle]
pub extern "C" fn mpenter() {
    switchkvm();
    seginit();
    lapicinit();
    unsafe {
        mpmain();
    }
}

#[no_mangle]
pub extern "C" fn startothers() {
    extern "C" {
        static _binary_out_entryother_start: u8;
        static _binary_out_entryother_size: u8;
    }

    let code = p2v(0x7000) as *mut u8;
    unsafe {
        core::ptr::copy_nonoverlapping(
            &_binary_out_entryother_start,
            code,
            &_binary_out_entryother_size as *const _ as usize,
        );
    }

    for c in cpus().iter() {
        if c == &cpu() {
            continue;
        }

        let stack = kalloc();
        unsafe {
            #[cfg(target_arch = "x86_64")]
            {
                *(code.offset(-4) as *mut u32) = 0x8000;
                *(code.offset(-8) as *mut u32) = v2p(entry32mp as *const () as usize) as u32;
                *(code.offset(-16) as *mut u64) = (stack as usize + KSTACKSIZE) as u64;
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                *(code.offset(-4) as *mut *mut u8) = stack.offset(KSTACKSIZE as isize);
                *(code.offset(-8) as *mut *mut u8) = mpenter as *mut u8;
                *(code.offset(-12) as *mut *mut u8) = v2p(entrypgdir.as_ptr() as usize) as *mut u8;
            }
        }

        lapicstartap(c.apicid, v2p(code as usize));

        while c.started.load(Ordering::SeqCst) == 0 {}
    }
}