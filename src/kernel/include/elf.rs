// src/include/elf.rs

pub const ELF_MAGIC: u32 = 0x464C457F; // "\x7FELF" in little endian

// File header
#[repr(C)]
pub struct ElfHdr {
    pub magic: u32,  // must equal ELF_MAGIC
    pub elf: [u8; 12],
    pub elf_type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: usize,
    pub phoff: usize,
    pub shoff: usize,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

// Program section header
#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct ProgHdr {
    pub prog_type: u32,
    pub flags: u32,
    pub off: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub align: u64,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct ProgHdr {
    pub prog_type: u32,
    pub off: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub filesz: u32,
    pub memsz: u32,
    pub flags: u32,
    pub align: u32,
}

// Values for ProgHdr type
pub const ELF_PROG_LOAD: u32 = 1;

// Flag bits for ProgHdr flags
pub const ELF_PROG_FLAG_EXEC: u32 = 1;
pub const ELF_PROG_FLAG_WRITE: u32 = 2;
pub const ELF_PROG_FLAG_READ: u32 = 4;