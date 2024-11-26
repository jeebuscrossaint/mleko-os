// src/kernel/kbd.rs

use crate::include::types::*;
use crate::include::x86::*;
use crate::include::defs::*;
use crate::include::kbd::*;

static mut SHIFT: u32 = 0;
static CHARCODE: [&[u8]; 4] = [NORMALMAP, SHIFTMAP, CTLMAP, CTLMAP];

pub fn kbdgetc() -> i32 {
    unsafe {
        let st = inb(KBSTATP);
        if (st & KBS_DIB) == 0 {
            return -1;
        }
        let mut data = inb(KBDATAP);

        if data == 0xE0 {
            SHIFT |= E0ESC;
            return 0;
        } else if data & 0x80 != 0 {
            // Key released
            data = if SHIFT & E0ESC != 0 { data } else { data & 0x7F };
            SHIFT &= !(SHIFTCODE[data as usize] | E0ESC);
            return 0;
        } else if SHIFT & E0ESC != 0 {
            // Last character was an E0 escape; or with 0x80
            data |= 0x80;
            SHIFT &= !E0ESC;
        }

        SHIFT |= SHIFTCODE[data as usize];
        SHIFT ^= TOGGLECODE[data as usize];
        let mut c = CHARCODE[(SHIFT & (CTL | SHIFT)) as usize][data as usize];
        if SHIFT & CAPSLOCK != 0 {
            if ('a' as u8) <= c && c <= ('z' as u8) {
                c = c - ('a' as u8) + ('A' as u8);
            } else if ('A' as u8) <= c && c <= ('Z' as u8) {
                c = c - ('A' as u8) + ('a' as u8);
            }
        }
        c as i32
    }
}

pub fn kbdintr() {
    consoleintr(kbdgetc);
}