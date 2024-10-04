// src/memlayout.rs

pub const EXTMEM: usize = 0x100000;            // Start of extended memory
pub const PHYSTOP: usize = 0xE000000;          // Top physical memory
pub const DEVSPACE: usize = 0xFE000000;        // Other devices are at high addresses

// Key addresses for address space layout (see kmap in vm.c for layout)
#[cfg(target_pointer_width = "64")]
pub const KERNBASE: usize = 0xFFFFFFFF80000000; // First kernel virtual address
#[cfg(target_pointer_width = "64")]
pub const DEVBASE: usize = 0xFFFFFFFF40000000;  // First device virtual address

#[cfg(target_pointer_width = "32")]
pub const KERNBASE: usize = 0x80000000;         // First kernel virtual address
#[cfg(target_pointer_width = "32")]
pub const DEVBASE: usize = 0xFE000000;          // First device virtual address

pub const KERNLINK: usize = KERNBASE + EXTMEM;  // Address where kernel is linked

// Functions to convert virtual addresses to physical addresses and vice versa
#[inline]
pub fn v2p(a: *const u8) -> usize {
    (a as usize) - KERNBASE
}

#[inline]
pub fn p2v(a: usize) -> *const u8 {
    (a + KERNBASE) as *const u8
}

// Macros as functions
#[inline]
pub fn v2p_wo(x: usize) -> usize {
    x - KERNBASE
}

#[inline]
pub fn p2v_wo(x: usize) -> usize {
    x + KERNBASE
}

#[inline]
pub fn io2v(a: usize) -> *const u8 {
    (a + DEVBASE - DEVSPACE) as *const u8
}