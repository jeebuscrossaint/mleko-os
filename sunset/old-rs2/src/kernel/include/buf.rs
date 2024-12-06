// include/buf.rs

use std::ptr::null_mut;

pub const B_BUSY: u32 = 0x1;  // buffer is locked by some process
pub const B_VALID: u32 = 0x2; // buffer has been read from disk
pub const B_DIRTY: u32 = 0x4; // buffer needs to be written to disk

pub const BSIZE: usize = 512; // Assuming BSIZE is defined somewhere

#[repr(C)]
pub struct Buf {
    pub flags: u32,
    pub dev: u32,
    pub blockno: u32,
    pub prev: *mut Buf, // LRU cache list
    pub next: *mut Buf,
    pub qnext: *mut Buf, // disk queue
    pub data: [u8; BSIZE],
}

impl Buf {
    pub fn new() -> Self {
        Buf {
            flags: 0,
            dev: 0,
            blockno: 0,
            prev: null_mut(),
            next: null_mut(),
            qnext: null_mut(),
            data: [0; BSIZE],
        }
    }

    // Example method to set flags
    pub fn set_flag(&mut self, flag: u32) {
        self.flags |= flag;
    }

    // Example method to clear flags
    pub fn clear_flag(&mut self, flag: u32) {
        self.flags &= !flag;
    }

    // Example method to check if a flag is set
    pub fn is_flag_set(&self, flag: u32) -> bool {
        self.flags & flag != 0
    }
}