// src/kernel/acpi.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::memlayout::*;
use crate::include::mp::*;
use crate::include::x86::*;
use crate::include::mmu::*;
use crate::include::proc::*;
use crate::include::acpi::*;

extern "C" {
    static mut cpus: [Cpu; NCPU];
    static mut ismp: i32;
    static mut ncpu: i32;
    static mut ioapicid: u8;
}

fn scan_rdsp(base: u32, len: u32) -> Option<&'static AcpiRdsp> {
    let mut p = p2v(base) as *const u8;
    let mut len = len;

    while len >= core::mem::size_of::<AcpiRdsp>() as u32 {
        if unsafe { core::ptr::read(p) } == SIG_RDSP {
            let mut sum = 0;
            for n in 0..20 {
                sum += unsafe { *p.add(n) } as u32;
            }
            if (sum & 0xff) == 0 {
                return Some(unsafe { &*(p as *const AcpiRdsp) });
            }
        }
        len -= 4;
        p = unsafe { p.add(4) };
    }
    None
}

fn find_rdsp() -> Option<&'static AcpiRdsp> {
    let pa = unsafe { *(P2V(0x40E) as *const u16) } as u32 * 16; // EBDA
    if pa != 0 {
        if let Some(rdsp) = scan_rdsp(pa, 1024) {
            return Some(rdsp);
        }
    }
    scan_rdsp(0xE0000, 0x20000)
}

fn acpi_config_smp(madt: &AcpiMadt) -> i32 {
    let lapic_addr = madt.lapic_addr_phys;
    let mut nioapic = 0;

    let p = madt.table.as_ptr();
    let e = unsafe { p.add(madt.header.length as usize - core::mem::size_of::<AcpiMadt>()) };

    let mut p = p;
    while p < e {
        let len = unsafe { *p.add(1) } as usize;
        if (e as usize - p as usize) < len {
            break;
        }
        match unsafe { *p } {
            TYPE_LAPIC => {
                let lapic = unsafe { &*(p as *const MadtLapic) };
                if len < core::mem::size_of::<MadtLapic>() {
                    break;
                }
                if (lapic.flags & APIC_LAPIC_ENABLED) == 0 {
                    break;
                }
                println!("acpi: cpu#{} apicid {}", unsafe { ncpu }, lapic.apic_id);
                unsafe {
                    cpus[ncpu as usize].id = ncpu;
                    cpus[ncpu as usize].apicid = lapic.apic_id;
                    ncpu += 1;
                }
            }
            TYPE_IOAPIC => {
                let ioapic = unsafe { &*(p as *const MadtIoapic) };
                if len < core::mem::size_of::<MadtIoapic>() {
                    break;
                }
                println!(
                    "acpi: ioapic#{} @{:x} id={} base={}",
                    nioapic, ioapic.addr, ioapic.id, ioapic.interrupt_base
                );
                if nioapic != 0 {
                    println!("warning: multiple ioapics are not supported");
                } else {
                    unsafe {
                        ioapicid = ioapic.id;
                    }
                }
                nioapic += 1;
            }
            _ => {}
        }
        p = unsafe { p.add(len) };
    }

    if unsafe { ncpu } != 0 {
        unsafe {
            ismp = 1;
            lapic = IO2V(lapic_addr as usize);
        }
        return 0;
    }

    -1
}

#[cfg(target_arch = "x86_64")]
const PHYSLIMIT: u32 = 0x80000000;
#[cfg(not(target_arch = "x86_64"))]
const PHYSLIMIT: u32 = 0x0E000000;

pub fn acpiinit() -> i32 {
    let rdsp = find_rdsp().expect("Failed to find RDSP");
    if rdsp.rsdt_addr_phys > PHYSLIMIT {
        println!("acpi: tables above 0x{:x} not mapped.", PHYSLIMIT);
        return -1;
    }
    let rsdt = p2v(rdsp.rsdt_addr_phys) as *const AcpiRsdt;
    let count = (unsafe { (*rsdt).header.length } as usize - core::mem::size_of::<AcpiRsdt>()) / 4;
    let entries = unsafe { slice::from_raw_parts((*rsdt).entry.as_ptr(), count) };

    for &entry in entries {
        if entry > PHYSLIMIT {
            println!("acpi: tables above 0x{:x} not mapped.", PHYSLIMIT);
            return -1;
        }
        let hdr = p2v(entry) as *const AcpiDescHeader;
        if unsafe { core::ptr::read(hdr).signature } == SIG_MADT {
            let madt = unsafe { &*(hdr as *const AcpiMadt) };
            return acpi_config_smp(madt);
        }
    }

    -1
}