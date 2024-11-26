// References: ACPI 6.5
// https://uefi.org/specs/ACPI/6.5/

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiRdsp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_addr_phys: u32,
    length: u32,
    xsdt_addr_phys: u64,
    xchecksum: u8,
    reserved: [u8; 3],
    reserved2: u32, // New field in ACPI 6.5
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiDescHeader {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_tableid: [u8; 8],
    oem_revision: u32,
    creator_id: [u8; 4],
    creator_revision: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiRsdt {
    header: AcpiDescHeader,
    entry: [u32; 0],
}

pub const TYPE_LAPIC: u8 = 0;
pub const TYPE_IOAPIC: u8 = 1;
pub const TYPE_INT_SRC_OVERRIDE: u8 = 2;
pub const TYPE_NMI_INT_SRC: u8 = 3;
pub const TYPE_LAPIC_NMI: u8 = 4;

pub const SIG_MADT: &str = "APIC";

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiMadt {
    header: AcpiDescHeader,
    lapic_addr_phys: u32,
    flags: u32,
    table: [u8; 0],
}

pub const APIC_LAPIC_ENABLED: u32 = 1;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MadtLapic {
    type_: u8,
    length: u8,
    acpi_id: u8,
    apic_id: u8,
    flags: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct MadtIoapic {
    type_: u8,
    length: u8,
    id: u8,
    reserved: u8,
    addr: u32,
    interrupt_base: u32,
}

// Example of a new structure in ACPI 6.5
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AcpiPccSubspace {
    type_: u8,
    length: u8,
    reserved: [u8; 6],
    base_address: u64,
    length_: u64,
    doorbell_register: u32,
    doorbell_preserve: u32,
    doorbell_write: u32,
    latency: u32,
    max_access_rate: u32,
    min_turnaround_time: u16,
}