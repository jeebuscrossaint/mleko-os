// src/stat.rs

// File types
pub const T_DIR: i16 = 1;   // Directory
pub const T_FILE: i16 = 2;  // File
pub const T_DEV: i16 = 3;   // Device

// File status structure
#[repr(C)]
pub struct Stat {
    pub type_: i16,      // Type of file
    pub dev: i32,        // File system's disk device
    pub ino: u32,        // Inode number
    pub nlink: i16,      // Number of links to file
    pub ownerid: i16,    // The owner of the file
    pub groupid: i16,    // The group owner of the file
    pub mode: u32,       // The permissions mode of the file
    pub size: u32,       // Size of file in bytes
}