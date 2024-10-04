// src/bootmain.rs

use crate::include::types::*;
use crate::include::x86::*;
use crate::include::memlayout::*;

const SECTSIZE: u32 = 512;

#[repr(C)]
struct MbHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
    header_addr: u32,
    load_addr: u32,
    load_end_addr: u32,
    bss_end_addr: u32,
    entry_addr: u32,
}

fn readseg(pa: *mut u8, count: u32, offset: u32) {
    let mut pa = pa;
    let epa = unsafe { pa.add(count as usize) };

    // Round down to sector boundary.
    pa = unsafe { pa.sub((offset % SECTSIZE) as usize) };

    // Translate from bytes to sectors; kernel starts at sector 1.
    let mut offset = (offset / SECTSIZE) + 1;

    // If this is too slow, we could read lots of sectors at a time.
    // We'd write more to memory than asked, but it doesn't matter --
    // we load in increasing order.
    while pa < epa {
        readsect(pa, offset);
        pa = unsafe { pa.add(SECTSIZE as usize) };
        offset += 1;
    }
}

pub fn bootmain() {
    let x = 0x10000 as *mut u32; // scratch space

    // multiboot header must be in the first 8192 bytes
    readseg(x as *mut u8, 8192, 0);

    for n in 0..(8192 / 4) {
        if unsafe { *x.add(n) } == 0x1BADB002 {
            if unsafe { *x.add(n) + *x.add(n + 1) + *x.add(n + 2) } == 0 {
                let hdr = unsafe { &*(x.add(n) as *const MbHeader) };

                if (hdr.flags & 0x10000) == 0 {
                    return; // does not have load_* fields, cannot proceed
                }
                if hdr.load_addr > hdr.header_addr {
                    return; // invalid;
                }
                if hdr.load_end_addr < hdr.load_addr {
                    return; // no idea how much to load
                }

                readseg(
                    hdr.load_addr as *mut u8,
                    hdr.load_end_addr - hdr.load_addr,
                    (n * 4) as u32 - (hdr.header_addr - hdr.load_addr),
                );

                // If too much RAM was allocated, then zero redundant RAM
                if hdr.bss_end_addr > hdr.load_end_addr {
                    stosb(
                        hdr.load_end_addr as *mut u8,
                        0,
                        hdr.bss_end_addr - hdr.load_end_addr,
                    );
                }

                // Call the entry point from the multiboot header.
                // Does not return!
                let entry: extern "C" fn() = unsafe { core::mem::transmute(hdr.entry_addr) };
                entry();
            }
        }
    }
}

fn waitdisk() {
    // Wait for disk ready.
    while (inb(0x1F7) & 0xC0) != 0x40 {}
}

// Read a single sector at offset into dst.
fn readsect(dst: *mut u8, offset: u32) {
    // Issue command.
    waitdisk();
    outb(0x1F2, 1); // count = 1
    outb(0x1F3, offset as u8);
    outb(0x1F4, (offset >> 8) as u8);
    outb(0x1F5, (offset >> 16) as u8);
    outb(0x1F6, ((offset >> 24) as u8) | 0xE0);
    outb(0x1F7, 0x20); // cmd 0x20 - read sectors

    // Read data.
    waitdisk();
    insl(0x1F0, dst, (SECTSIZE / 4) as usize);
}