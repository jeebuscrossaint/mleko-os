// src/mmu.rs

// Eflags register
pub const FL_CF: u32 = 0x00000001;      // Carry Flag
pub const FL_PF: u32 = 0x00000004;      // Parity Flag
pub const FL_AF: u32 = 0x00000010;      // Auxiliary carry Flag
pub const FL_ZF: u32 = 0x00000040;      // Zero Flag
pub const FL_SF: u32 = 0x00000080;      // Sign Flag
pub const FL_TF: u32 = 0x00000100;      // Trap Flag
pub const FL_IF: u32 = 0x00000200;      // Interrupt Enable
pub const FL_DF: u32 = 0x00000400;      // Direction Flag
pub const FL_OF: u32 = 0x00000800;      // Overflow Flag
pub const FL_IOPL_MASK: u32 = 0x00003000; // I/O Privilege Level bitmask
pub const FL_IOPL_0: u32 = 0x00000000;  //   IOPL == 0
pub const FL_IOPL_1: u32 = 0x00001000;  //   IOPL == 1
pub const FL_IOPL_2: u32 = 0x00002000;  //   IOPL == 2
pub const FL_IOPL_3: u32 = 0x00003000;  //   IOPL == 3
pub const FL_NT: u32 = 0x00004000;      // Nested Task
pub const FL_RF: u32 = 0x00010000;      // Resume Flag
pub const FL_VM: u32 = 0x00020000;      // Virtual 8086 mode
pub const FL_AC: u32 = 0x00040000;      // Alignment Check
pub const FL_VIF: u32 = 0x00080000;     // Virtual Interrupt Flag
pub const FL_VIP: u32 = 0x00100000;     // Virtual Interrupt Pending
pub const FL_ID: u32 = 0x00200000;      // ID flag

// Control Register flags
pub const CR0_PE: u32 = 0x00000001;     // Protection Enable
pub const CR0_MP: u32 = 0x00000002;     // Monitor coProcessor
pub const CR0_EM: u32 = 0x00000004;     // Emulation
pub const CR0_TS: u32 = 0x00000008;     // Task Switched
pub const CR0_ET: u32 = 0x00000010;     // Extension Type
pub const CR0_NE: u32 = 0x00000020;     // Numeric Errror
pub const CR0_WP: u32 = 0x00010000;     // Write Protect
pub const CR0_AM: u32 = 0x00040000;     // Alignment Mask
pub const CR0_NW: u32 = 0x20000000;     // Not Writethrough
pub const CR0_CD: u32 = 0x40000000;     // Cache Disable
pub const CR0_PG: u32 = 0x80000000;     // Paging

pub const CR4_PSE: u32 = 0x00000010;    // Page size extension

pub const SEG_KCODE: u32 = 1;  // kernel code
pub const SEG_KDATA: u32 = 2;  // kernel data+stack
pub const SEG_KCPU: u32 = 3;   // kernel per-cpu data
pub const SEG_UCODE: u32 = 4;  // user code
pub const SEG_UDATA: u32 = 5;  // user data+stack
pub const SEG_TSS: u32 = 6;    // this process's task state

pub const DPL_USER: u32 = 0x3; // User DPL

// Application segment type bits
pub const STA_X: u32 = 0x8;    // Executable segment
pub const STA_E: u32 = 0x4;    // Expand down (non-executable segments)
pub const STA_C: u32 = 0x4;    // Conforming code segment (executable only)
pub const STA_W: u32 = 0x2;    // Writeable (non-executable segments)
pub const STA_R: u32 = 0x2;    // Readable (executable segments)
pub const STA_A: u32 = 0x1;    // Accessed

// System segment type bits
pub const STS_T16A: u32 = 0x1; // Available 16-bit TSS
pub const STS_LDT: u32 = 0x2;  // Local Descriptor Table
pub const STS_T16B: u32 = 0x3; // Busy 16-bit TSS
pub const STS_CG16: u32 = 0x4; // 16-bit Call Gate
pub const STS_TG: u32 = 0x5;   // Task Gate / Coum Transmitions
pub const STS_IG16: u32 = 0x6; // 16-bit Interrupt Gate
pub const STS_TG16: u32 = 0x7; // 16-bit Trap Gate
pub const STS_T32A: u32 = 0x9; // Available 32-bit TSS
pub const STS_T32B: u32 = 0xB; // Busy 32-bit TSS
pub const STS_CG32: u32 = 0xC; // 32-bit Call Gate
pub const STS_IG32: u32 = 0xE; // 32-bit Interrupt Gate
pub const STS_TG32: u32 = 0xF; // 32-bit Trap Gate

// Page directory and page table constants.
#[cfg(target_pointer_width = "64")]
pub const NPDENTRIES: usize = 512;     // # directory entries per page directory
#[cfg(target_pointer_width = "64")]
pub const NPTENTRIES: usize = 512;     // # PTEs per page table
#[cfg(target_pointer_width = "64")]
pub const PGSIZE: usize = 4096;        // bytes mapped by a page

#[cfg(target_pointer_width = "64")]
pub const PGSHIFT: usize = 12;         // log2(PGSIZE)
#[cfg(target_pointer_width = "64")]
pub const PTXSHIFT: usize = 12;        // offset of PTX in a linear address
#[cfg(target_pointer_width = "64")]
pub const PDXSHIFT: usize = 21;        // offset of PDX in a linear address

#[cfg(target_pointer_width = "64")]
pub const PXMASK: usize = 0x1FF;

#[cfg(target_pointer_width = "32")]
pub const NPDENTRIES: usize = 1024;    // # directory entries per page directory
#[cfg(target_pointer_width = "32")]
pub const NPTENTRIES: usize = 1024;    // # PTEs per page table
#[cfg(target_pointer_width = "32")]
pub const PGSIZE: usize = 4096;        // bytes mapped by a page

#[cfg(target_pointer_width = "32")]
pub const PGSHIFT: usize = 12;         // log2(PGSIZE)
#[cfg(target_pointer_width = "32")]
pub const PTXSHIFT: usize = 12;        // offset of PTX in a linear address
#[cfg(target_pointer_width = "32")]
pub const PDXSHIFT: usize = 22;        // offset of PDX in a linear address

#[cfg(target_pointer_width = "32")]
pub const PXMASK: usize = 0x3FF;

pub const PTE_P: u32 = 0x001;   // Present
pub const PTE_W: u32 = 0x002;   // Writeable
pub const PTE_U: u32 = 0x004;   // User
pub const PTE_PWT: u32 = 0x008; // Write-Through
pub const PTE_PCD: u32 = 0x010; // Cache-Disable
pub const PTE_A: u32 = 0x020;   // Accessed
pub const PTE_D: u32 = 0x040;   // Dirty
pub const PTE_PS: u32 = 0x080;  // Page Size
pub const PTE_MBZ: u32 = 0x180; // Bits must be zero

#[inline]
pub fn pte_addr(pte: usize) -> usize {
    pte & !0xFFF
}

#[inline]
pub fn pte_flags(pte: usize) -> usize {
    pte & 0xFFF
}

#[inline]
pub fn pdx(va: usize) -> usize {
    (va >> PDXSHIFT) & PXMASK
}

#[inline]
pub fn ptx(va: usize) -> usize {
    (va >> PTXSHIFT) & PXMASK
}

#[inline]
pub fn pgaddr(d: usize, t: usize, o: usize) -> usize {
    (d << PDXSHIFT) | (t << PTXSHIFT) | o
}

#[inline]
pub fn pgroundup(sz: usize) -> usize {
    (sz + PGSIZE - 1) & !(PGSIZE - 1)
}

#[inline]
pub fn pgrounddown(a: usize) -> usize {
    a & !(PGSIZE - 1)
}

#[repr(C)]
pub struct Segdesc {
    lim_15_0: u16,  // Low bits of segment limit
    base_15_0: u16, // Low bits of segment base address
    base_23_16: u8, // Middle bits of segment base address
    type_: u8,      // Segment type (see STS_ constants)
    s: u8,          // 0 = system, 1 = application
    dpl: u8,        // Descriptor Privilege Level
    p: u8,          // Present
    lim_19_16: u8,  // High bits of segment limit
    avl: u8,        // Unused (available for software use)
    rsv1: u8,       // Reserved
    db: u8,         // 0 = 16-bit segment, 1 = 32-bit segment
    g: u8,          // Granularity: limit scaled by 4K when set
    base_31_24: u8, // High bits of segment base address
}

impl Segdesc {
    pub fn new(type_: u8, base: usize, lim: usize, dpl: u8) -> Self {
        Segdesc {
            lim_15_0: ((lim >> 12) & 0xffff) as u16,
            base_15_0: (base & 0xffff) as u16,
            base_23_16: ((base >> 16) & 0xff) as u8,
            type_,
            s: 1,
            dpl,
            p: 1,
            lim_19_16: ((lim >> 28) & 0xf) as u8,
            avl: 0,
            rsv1: 0,
            db: 1,
            g: 1,
            base_31_24: ((base >> 24) & 0xff) as u8,
        }
    }

    pub fn new16(type_: u8, base: usize, lim: usize, dpl: u8) -> Self {
        Segdesc {
            lim_15_0: (lim & 0xffff) as u16,
            base_15_0: (base & 0xffff) as u16,
            base_23_16: ((base >> 16) & 0xff) as u8,
            type_,
            s: 1,
            dpl,
            p: 1,
            lim_19_16: ((lim >> 16) & 0xf) as u8,
            avl: 0,
            rsv1: 0,
            db: 1,
            g: 0,
            base_31_24: ((base >> 24) & 0xff) as u8,
        }
    }
}

#[repr(C)]
pub struct Taskstate {
    pub link: u32,         // Old ts selector
    pub esp0: u32,         // Stack pointers and segment selectors
    pub ss0: u16,          //   after an increase in privilege level
    pub padding1: u16,
    pub esp1: *mut u32,
    pub ss1: u16,
    pub padding2: u16,
    pub esp2: *mut u32,
    pub ss2: u16,
    pub padding3: u16,
    pub cr3: *mut u8,      // Page directory base
    pub eip: *mut u32,     // Saved state from last task switch
    pub eflags: u32,
    pub eax: u32,          // More saved state (registers)
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: *mut u32,
    pub ebp: *mut u32,
    pub esi: u32,
    pub edi: u32,
    pub es: u16,           // Even more saved state (segment selectors)
    pub padding4: u16,
    pub cs: u16,
    pub padding5: u16,
    pub ss: u16,
    pub padding6: u16,
    pub ds: u16,
    pub padding7: u16,
    pub fs: u16,
    pub padding8: u16,
    pub gs: u16,
    pub padding9: u16,
    pub ldt: u16,
    pub padding10: u16,
    pub t: u16,            // Trap on task switch
    pub iomb: u16,         // I/O map base address
}

#[repr(C)]
pub struct Gatedesc {
    pub off_15_0: u16,   // low 16 bits of offset in segment
    pub cs: u16,         // code segment selector
    pub args: u8,        // # args, 0 for interrupt/trap gates
    pub rsv1: u8,        // reserved(should be zero I guess)
    pub type_: u8,       // type(STS_{TG,IG32,TG32})
    pub s: u8,           // must be 0 (system)
    pub dpl: u8,         // descriptor(meaning new) privilege level
    pub p: u8,           // Present
    pub off_31_16: u16,  // high bits of offset in segment
}

impl Gatedesc {
    pub fn set_gate(&mut self, istrap: bool, sel: u16, off: usize, d: u8) {
        self.off_15_0 = (off & 0xffff) as u16;
        self.cs = sel;
        self.args = 0;
        self.rsv1 = 0;
        self.type_ = if istrap { STS_TG32 } else { STS_IG32 } as u8;
        self.s = 0;
        self.dpl = d;
        self.p = 1;
        self.off_31_16 = ((off >> 16) & 0xffff) as u16;
    }
}