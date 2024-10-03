// src/include/file.rs

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    None,
    Pipe,
    Inode,
}

pub struct File {
    pub file_type: FileType,
    pub ref_count: i32, // reference count
    pub readable: bool,
    pub writable: bool,
    pub pipe: Option<*mut Pipe>,
    pub ip: Option<*mut Inode>,
    pub off: u32,
}

// In-memory copy of an inode
pub struct Inode {
    pub dev: u32,           // Device number
    pub inum: u32,          // Inode number
    pub ref_count: i32,     // Reference count
    pub flags: i32,         // I_BUSY, I_VALID

    pub type_: i16,         // copy of disk inode
    pub major: i16,
    pub minor: i16,
    pub nlink: i16,
    pub ownerid: i16,       // The ID of the user who owns the file.
    pub groupid: i16,       // The ID of the group who owns the file.
    pub mode: u32,          // The file's mode e.g. 0700
    pub size: u32,
    pub addrs: [u32; NDIRECT + 1],
}

pub const I_BUSY: i32 = 0x1;
pub const I_VALID: i32 = 0x2;

// Table mapping major device number to device functions
pub struct Devsw {
    pub read: Option<fn(ip: &mut Inode, buf: &mut [u8], n: i32) -> i32>,
    pub write: Option<fn(ip: &mut Inode, buf: &mut [u8], n: i32) -> i32>,
}

extern "C" {
    pub static mut devsw: [Devsw; 256];
}

// Major device numbers
pub const CONSOLE: u32 = 1;
pub const CPUID: u32 = 2;

// Constants for NDIRECT
pub const NDIRECT: usize = 12;

// Dummy definitions for Pipe and NDIRECT to make the code compile
pub struct Pipe;