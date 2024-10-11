// src/kernel/kalloc.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::memlayout::*;
use crate::include::mmu::*;
use crate::include::spinlock::*;
use core::ptr;
use core::mem::size_of;

extern "C" {
    static end: u8; // first address after kernel loaded from ELF file
}

struct Run {
    next: Option<&'static mut Run>,
}

struct KMem {
    lock: Spinlock,
    use_lock: bool,
    freelist: Option<&'static mut Run>,
}

static mut KMEM: Kmem = Kmem {
    lock: Spinlock::new("kmem"),
    use_lock: false,
    freelist: None,
};

// init is in 2 phases (faze clan)
// 1. call kinit1 while still using entrypgdir to place just the pages
// the pages mapped by entrypgdir on free list
// 2. call kinit2 with the rest of the physical pages
// after installing a full page table that maps them on all corse.
pub fn kinit1(vstart: *mut u8, vend: *mut u8) {
    unsafe {
        KMEM.lock.init("kmem");
        KMEM.use_lock = false;
        freerange(vstart, vend);
    }
}

pub fn kinit2(vstart: *mut u8, vend: *mut u8) {
    unsafe {
        freerange(vstart, vend);
        KMEM.use_lock = true;
    }
}

fn freerange(vstart: *mut u8, vend: *mut u8) {
    let mut p = PGROUNDUP(vstart as usize) as *mut u8;
    while (p as usize) + PGSIZE <= vend as usize {
        kfree(p);
        p = p.add(PGSIZE);
    }
}

// Free the page of physical memory pointed at by v,
// which normally should have been returned by a
// call to kalloc().  (The exception is when
// initializing the allocator; see kinit above.)
pub fn kfree(v: *mut u8) {
    unsafe {
        if (v as usize) % PGSIZE != 0 || v < &end || v2p(v) >= PHYSTOP {
            panic("kfree");
        }

        // Fill with junk to catch dangling refs.
        ptr::write_bytes(v, 1, PGSIZE);

        if KMEM.use_lock {
            KMEM.lock.acquire();
        }
        let r = v as *mut Run;
        (*r).next = KMEM.freelist.take();
        KMEM.freelist = Some(&mut *r);
        if KMEM.use_lock {
            KMEM.lock.release();
        }
    }
}

// Allocate one 4096-byte page of physical memory.
// Returns a pointer that the kernel can use.
// Returns 0 if the memory cannot be allocated.
pub fn kalloc() -> *mut u8 {
    unsafe {
        if KMEM.use_lock {
            KMEM.lock.acquire();
        }
        let r = KMEM.freelist.take();
        if let Some(run) = r {
            KMEM.freelist = run.next.take();
        }
        if KMEM.use_lock {
            KMEM.lock.release();
        }
        r.map_or(ptr::null_mut(), |run| run as *mut Run as *mut u8)
    }
}