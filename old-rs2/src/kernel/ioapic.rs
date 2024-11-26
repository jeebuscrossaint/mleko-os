// src/kernel/ioapic.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::traps::*;
use crate::include::memlayout::*;

const IOAPIC: u32 = 0xFEC00000; // Default physical address of IO APIC

const REG_ID: u32 = 0x00;     // Register index: ID
const REG_VER: u32 = 0x01;    // Register index: version
const REG_TABLE: u32 = 0x10;  // Redirection table base

// The redirection table starts at REG_TABLE and uses
// two registers to configure each interrupt.  
// The first (low) register in a pair contains configuration bits.
// The second (high) register contains a bitmask telling which
// CPUs can serve that interrupt.

const INT_DISABLED: u32 = 0x00010000  // Interrupt disabled
const INT_LEVEL: u32 = 0x00008000  // Level-triggered (vs edge-)
const INT_ACTIVELOW: u32 = 0x00002000  // Active low (vs high)
const INT_LOGICAL: u32 = 0x00000800  // Destination is CPU id (vs APIC ID)


use std::sync::atomic::{AtomicU32, Ordering};

// IO APIC MMIO structure: write reg, then read or write data.
#[repr(C)]
struct IoApic {
    reg: AtomicU32,
    _pad: [u32; 3],
    data: AtomicU32,
}

static mut IOAPIC: Option<&'static mut IoApic> = None;

const IO2V: usize = 0; // You'll need to define this constant
const IOAPIC_ADDR: usize = 0; // You'll need to define this constant
const REG_VER: u32 = 0x01;
const REG_ID: u32 = 0x00;
const REG_TABLE: u32 = 0x10;
const INT_DISABLED: u32 = 0x00010000;
const T_IRQ0: u32 = 32;

static mut IS_MP: bool = false;
static mut IOAPIC_ID: u32 = 0;

fn ioapic_read(reg: u32) -> u32 {
    unsafe {
        IOAPIC.as_ref().unwrap().reg.store(reg, Ordering::SeqCst);
        IOAPIC.as_ref().unwrap().data.load(Ordering::SeqCst)
    }
}

fn ioapic_write(reg: u32, data: u32) {
    unsafe {
        IOAPIC.as_ref().unwrap().reg.store(reg, Ordering::SeqCst);
        IOAPIC.as_ref().unwrap().data.store(data, Ordering::SeqCst);
    }
}

pub fn ioapic_init() {
    unsafe {
        if !IS_MP {
            return;
        }

        IOAPIC = Some(&mut *(IO2V + IOAPIC_ADDR) as *mut IoApic);

        let maxintr = (ioapic_read(REG_VER) >> 16) & 0xFF;
        let id = ioapic_read(REG_ID) >> 24;

        if id != IOAPIC_ID {
            println!("ioapicinit: id isn't equal to ioapicid; not a MP");
        }

        // Mark all interrupts edge-triggered, active high, disabled,
        // and not routed to any CPUs.
        for i in 0..=maxintr {
            ioapic_write(REG_TABLE + 2 * i, INT_DISABLED | (T_IRQ0 + i));
            ioapic_write(REG_TABLE + 2 * i + 1, 0);
        }
    }
}

pub fn ioapic_enable(irq: u32, cpunum: u32) {
    unsafe {
        if !IS_MP {
            return;
        }

        // Mark interrupt edge-triggered, active high,
        // enabled, and routed to the given cpunum,
        // which happens to be that cpu's APIC ID.
        ioapic_write(REG_TABLE + 2 * irq, T_IRQ0 + irq);
        ioapic_write(REG_TABLE + 2 * irq + 1, cpunum << 24);
    }
}