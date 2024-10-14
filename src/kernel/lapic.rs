// src/kernel/lapic.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::date::*;
use crate::include::memlayout::*;
use crate::include::traps::*;
use crate::include::mmu::*;
use crate::include::x86::*;
use crate::include::param::*;
use crate::include::proc::*;

// Local APIC registers, divided by 4 for use as u32[] indices.
const ID: usize = 0x0020 / 4;   // ID
const VER: usize = 0x0030 / 4;  // Version
const TPR: usize = 0x0080 / 4;  // Task Priority
const EOI: usize = 0x00B0 / 4;  // EOI
const SVR: usize = 0x00F0 / 4;  // Spurious Interrupt Vector
const ENABLE: u32 = 0x00000100; // Unit Enable
const ESR: usize = 0x0280 / 4;  // Error Status
const ICRLO: usize = 0x0300 / 4; // Interrupt Command
const INIT: u32 = 0x00000500;   // INIT/RESET
const STARTUP: u32 = 0x00000600; // Startup IPI
const DELIVS: u32 = 0x00001000; // Delivery status
const ASSERT: u32 = 0x00004000; // Assert interrupt (vs deassert)
const DEASSERT: u32 = 0x00000000;
const LEVEL: u32 = 0x00008000;  // Level triggered
const BCAST: u32 = 0x00080000;  // Send to all APICs, including self.
const BUSY: u32 = 0x00001000;
const FIXED: u32 = 0x00000000;
const ICRHI: usize = 0x0310 / 4; // Interrupt Command [63:32]
const TIMER: usize = 0x0320 / 4; // Local Vector Table 0 (TIMER)
const X1: u32 = 0x0000000B;     // divide counts by 1
const PERIODIC: u32 = 0x00020000; // Periodic
const PCINT: usize = 0x0340 / 4; // Performance Counter LVT
const LINT0: usize = 0x0350 / 4; // Local Vector Table 1 (LINT0)
const LINT1: usize = 0x0360 / 4; // Local Vector Table 2 (LINT1)
const ERROR: usize = 0x0370 / 4; // Local Vector Table 3 (ERROR)
const MASKED: u32 = 0x00010000; // Interrupt masked
const TICR: usize = 0x0380 / 4; // Timer Initial Count
const TCCR: usize = 0x0390 / 4; // Timer Current Count
const TDCR: usize = 0x03E0 / 4; // Timer Divide Configuration

static mut LAPIC: *mut u32 = core::ptr::null_mut(); // Initialized in mp.rs

fn lapicw(index: usize, value: u32) {
    unsafe {
        if !LAPIC.is_null() {
            core::ptr::write_volatile(LAPIC.add(index), value);
            core::ptr::read_volatile(LAPIC.add(ID)); // wait for write to finish, by reading
        }
    }
}

pub fn lapicinit() {
    unsafe {
        if LAPIC.is_null() {
            return;
        }

        // Enable local APIC; set spurious interrupt vector.
        lapicw(SVR, ENABLE | (T_IRQ0 + IRQ_SPURIOUS) as u32);

        // The timer repeatedly counts down at bus frequency
        // from lapic[TICR] and then issues an interrupt.  
        // If xv6 cared more about precise timekeeping,
        // TICR would be calibrated using an external time source.
        lapicw(TDCR, X1);
        lapicw(TIMER, PERIODIC | (T_IRQ0 + IRQ_TIMER) as u32);
        lapicw(TICR, 10_000_000);

        // Disable logical interrupt lines.
        lapicw(LINT0, MASKED);
        lapicw(LINT1, MASKED);

        // Disable performance counter overflow interrupts
        // on machines that provide that interrupt entry.
        if (core::ptr::read_volatile(LAPIC.add(VER)) >> 16) & 0xFF >= 4 {
            lapicw(PCINT, MASKED);
        }

        // Map error interrupt to IRQ_ERROR.
        lapicw(ERROR, T_IRQ0 + IRQ_ERROR as u32);

        // Clear error status register (requires back-to-back writes).
        lapicw(ESR, 0);
        lapicw(ESR, 0);

        // Ack any outstanding interrupts.
        lapicw(EOI, 0);

        // Send an Init Level De-Assert to synchronize arbitration ID's.
        lapicw(ICRHI, 0);
        lapicw(ICRLO, BCAST | INIT | LEVEL);
        while core::ptr::read_volatile(LAPIC.add(ICRLO)) & DELIVS != 0 {}

        // Enable interrupts on the APIC (but not on the processor).
        lapicw(TPR, 0);
    }
}

pub fn cpunum() -> i32 {
    let mut n: i32;
    let id: i32;

    // Cannot call cpu when interrupts are enabled:
    // result not guaranteed to last long enough to be used!
    // Would prefer to panic but even printing is chancy here:
    // almost everything, including cprintf and panic, calls cpu,
    // often indirectly through acquire and release.
    if readeflags() & FL_IF != 0 {
        static mut N: i32 = 0;
        unsafe {
            if N == 0 {
                N += 1;
                println!(
                    "cpu called from {:p} with interrupts enabled",
                    __builtin_return_address(0)
                );
            }
        }
    }

    unsafe {
        if LAPIC.is_null() {
            return 0;
        }

        id = (core::ptr::read_volatile(LAPIC.add(ID)) >> 24) as i32;
        for n in 0..ncpu {
            if id == cpus[n as usize].apicid {
                return n;
            }
        }
    }

    0
}

extern "C" {
    fn __builtin_return_address(level: usize) -> *const u8;
}

pub fn lapiceoi() {
    unsafe {
        if !LAPIC.is_null() {
            lapicw(EOI, 0);
        }
    }
}

// Spin for a given number of microseconds.
// On real hardware would want to tune this dynamically.
pub fn microdelay(_us: i32) {
    // Implementation can be added if needed
}

const CMOS_PORT: u16 = 0x70;
const CMOS_RETURN: u16 = 0x71;

// Start additional processor running entry code at addr.
// See Appendix B of MultiProcessor Specification.
pub fn lapicstartap(apicid: u8, addr: u32) {
    let mut i: i32;
    let wrv: *mut u16;

    unsafe {
        // "The BSP must initialize CMOS shutdown code to 0AH
        // and the warm reset vector (DWORD based at 40:67) to point at
        // the AP startup code prior to the [universal startup algorithm]."
        outb(CMOS_PORT, 0xF); // offset 0xF is shutdown code
        outb(CMOS_PORT + 1, 0x0A);
        wrv = P2V((0x40 << 4 | 0x67) as usize) as *mut u16; // Warm reset vector
        *wrv.offset(0) = 0;
        *wrv.offset(1) = (addr >> 4) as u16;

        // "Universal startup algorithm."
        // Send INIT (level-triggered) interrupt to reset other CPU.
        lapicw(ICRHI, (apicid as u32) << 24);
        lapicw(ICRLO, INIT | LEVEL | ASSERT);
        microdelay(200);
        lapicw(ICRLO, INIT | LEVEL);
        microdelay(10000);

        // Send startup IPI (twice!) to enter code.
        // Regular hardware is supposed to only accept a STARTUP
        // when it is in the halted state due to an INIT. So the second
        // should be ignored, but it is part of the official Intel algorithm.
        for i in 0..2 {
            lapicw(ICRHI, (apicid as u32) << 24);
            lapicw(ICRLO, STARTUP | (addr >> 12));
            microdelay(200);
        }
    }
}

const CMOS_STATA: u8 = 0x0a;
const CMOS_STATB: u8 = 0x0b;
const CMOS_UIP: u8 = 1 << 7; // RTC update in progress

const SECS: u8 = 0x00;
const MINS: u8 = 0x02;
const HOURS: u8 = 0x04;
const DAY: u8 = 0x07;
const MONTH: u8 = 0x08;
const YEAR: u8 = 0x09;

fn cmos_read(reg: u8) -> u32 {
    unsafe {
        outb(CMOS_PORT, reg);
        microdelay(200);
        inb(CMOS_RETURN) as u32
    }
}

fn fill_rtcdate(r: &mut RtcDate) {
    r.second = cmos_read(SECS);
    r.minute = cmos_read(MINS);
    r.hour = cmos_read(HOURS);
    r.day = cmos_read(DAY);
    r.month = cmos_read(MONTH);
    r.year = cmos_read(YEAR);
}

// qemu seems to use 24-hour GWT and the values are BCD encoded
pub fn cmostime(r: &mut RtcDate) {
    let mut t1 = RtcDate::default();
    let mut t2 = RtcDate::default();
    let sb: i32;
    let bcd: bool;

    sb = cmos_read(CMOS_STATB) as i32;
    bcd = (sb & (1 << 2)) == 0;

    // make sure CMOS doesn't modify time while we read it
    loop {
        fill_rtcdate(&mut t1);
        if cmos_read(CMOS_STATA) & CMOS_UIP as u32 != 0 {
            continue;
        }
        fill_rtcdate(&mut t2);
        if t1 == t2 {
            break;
        }
    }

    // convert
    if bcd {
        t1.second = ((t1.second >> 4) * 10) + (t1.second & 0xf);
        t1.minute = ((t1.minute >> 4) * 10) + (t1.minute & 0xf);
        t1.hour = ((t1.hour >> 4) * 10) + (t1.hour & 0xf);
        t1.day = ((t1.day >> 4) * 10) + (t1.day & 0xf);
        t1.month = ((t1.month >> 4) * 10) + (t1.month & 0xf);
        t1.year = ((t1.year >> 4) * 10) + (t1.year & 0xf);
    }

    *r = t1;
    r.year += 2000;
}

#[derive(Default, PartialEq)]
pub struct RtcDate {
    pub second: u32,
    pub minute: u32,
    pub hour: u32,
    pub day: u32,
    pub month: u32,
    pub year: u32,
}