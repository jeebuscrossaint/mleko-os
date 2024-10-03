// src/include/fs.rs

pub const ROOTINO: u32 = 1;  // root i-number
pub const BSIZE: u32 = 512;  // block size

// File system super block
#[repr(C)]
pub struct Superblock {
    pub size: u32,         // Size of file system image (blocks)
    pub nblocks: u32,      // Number of data blocks
    pub ninodes: u32,      // Number of inodes.
    pub nlog: u32,         // Number of log blocks
}

pub const NDIRECT: usize = 10;
pub const NINDIRECT: usize = BSIZE as usize / std::mem::size_of::<u32>();
pub const MAXFILE: usize = NDIRECT + NINDIRECT;

// On-disk inode structure
#[repr(C)]
pub struct Dinode {
    pub type_: i16,           // File type
    pub major: i16,           // Major device number (T_DEV only)
    pub minor: i16,           // Minor device number (T_DEV only)
    pub nlink: i16,           // Number of links to inode in file system
    pub ownerid: i16,         // The ID of the user who owns the file.
    pub groupid: i16,         // The ID of the group who owns the file.
    pub mode: u32,            // The files mode e.g. 0700
    pub size: u32,            // Size of file (bytes)
    pub addrs: [u32; NDIRECT + 1],   // Data block addresses
}

// Inodes per block.
pub const IPB: usize = BSIZE as usize / std::mem::size_of::<Dinode>();

// Block containing inode i
pub fn iblock(i: usize) -> usize {
    i / IPB + 2
}

// Bitmap bits per block
pub const BPB: usize = BSIZE as usize * 8;

// Block containing bit for block b
pub fn bblock(b: usize, ninodes: usize) -> usize {
    b / BPB + ninodes / IPB + 3
}

// Directory is a file containing a sequence of dirent structures.
pub const DIRSIZ: usize = 14;

#[repr(C)]
pub struct Dirent {
    pub inum: u16,
    pub name: [u8; DIRSIZ],
}