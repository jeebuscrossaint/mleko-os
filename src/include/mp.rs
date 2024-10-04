// src/mp.rs

// Table entry types
pub const MPPROC: u8 = 0x00;    // One per processor
pub const MPBUS: u8 = 0x01;     // One per bus
pub const MPIOAPIC: u8 = 0x02;  // One per I/O APIC
pub const MPIOINTR: u8 = 0x03;  // One per bus interrupt source
pub const MPLINTR: u8 = 0x04;   // One per system interrupt source

// Processor flags
pub const MPBOOT: u8 = 0x02;    // This proc is the bootstrap processor

#[repr(C)]
pub struct Mp {
    pub signature: [u8; 4],  // "_MP_"
    pub physaddr: u32,       // phys addr of MP config table
    pub length: u8,          // 1
    pub specrev: u8,         // [14]
    pub checksum: u8,        // all bytes must add up to 0
    pub type_: u8,           // MP system config type
    pub imcrp: u8,
    pub reserved: [u8; 3],
}

#[repr(C)]
pub struct Mpconf {
    pub signature: [u8; 4],  // "PCMP"
    pub length: u16,         // total table length
    pub version: u8,         // [14]
    pub checksum: u8,        // all bytes must add up to 0
    pub product: [u8; 20],   // product id
    pub oemtable: u32,       // OEM table pointer
    pub oemlength: u16,      // OEM table length
    pub entry: u16,          // entry count
    pub lapicaddr: u32,      // address of local APIC
    pub xlength: u16,        // extended table length
    pub xchecksum: u8,       // extended table checksum
    pub reserved: u8,
}

#[repr(C)]
pub struct Mpproc {
    pub type_: u8,           // entry type (0)
    pub apicid: u8,          // local APIC id
    pub version: u8,         // local APIC version
    pub flags: u8,           // CPU flags
    pub signature: [u8; 4],  // CPU signature
    pub feature: u32,        // feature flags from CPUID instruction
    pub reserved: [u8; 8],
}

#[repr(C)]
pub struct Mpioapic {
    pub type_: u8,           // entry type (2)
    pub apicno: u8,          // I/O APIC id
    pub version: u8,         // I/O APIC version
    pub flags: u8,           // I/O APIC flags
    pub addr: u32,           // I/O APIC address
}