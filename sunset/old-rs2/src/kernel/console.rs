// src/console.rs

use core::fmt::Write;
use core::ptr;
use crate::include::types::*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::traps::*;
use crate::include::spinlock::*;
use crate::include::fs::*;
use crate::include::file::*;
use crate::include::memlayout::*;
use crate::include::mmu::*;
use crate::include::proc::*;
use crate::include::x86::*;

static mut PANICKED: bool = false;

static mut CONS: Console = Console {
    lock: Spinlock::new("console"),
    locking: false,
};

static DIGITS: &[u8] = b"0123456789abcdef";

struct Console {
    lock: Spinlock,
    locking: bool,
}

fn printptr(x: usize) {
    let mut x = x;
    for _ in 0..(core::mem::size_of::<usize>() * 2) {
        consputc(DIGITS[(x >> (core::mem::size_of::<usize>() * 8 - 4)) & 0xf] as i32);
        x <<= 4;
    }
}

fn printint(mut xx: i32, base: i32, sign: bool) {
    let mut buf = [0u8; 16];
    let mut i = 0;
    let mut x = if sign && xx < 0 {
        xx = -xx;
        xx as u32
    } else {
        xx as u32
    };

    loop {
        buf[i] = DIGITS[(x % base as u32) as usize];
        i += 1;
        x /= base as u32;
        if x == 0 {
            break;
        }
    }

    if sign && xx < 0 {
        buf[i] = b'-';
        i += 1;
    }

    while i > 0 {
        i -= 1;
        consputc(buf[i] as i32);
    }
}

pub fn cprintf(fmt: &str, args: fmt::Arguments) {
    let locking = unsafe { CONS.locking };
    if locking {
        unsafe { CONS.lock.acquire() };
    }

    if fmt.is_empty() {
        panic("null fmt");
    }

    let mut iter = fmt.chars();
    while let Some(c) = iter.next() {
        if c != '%' {
            consputc(c as i32);
            continue;
        }
        match iter.next().unwrap_or('\0') {
            'd' => printint(args.next().unwrap().as_i32().unwrap(), 10, true),
            'x' => printint(args.next().unwrap().as_i32().unwrap(), 16, false),
            'p' => printptr(args.next().unwrap().as_usize().unwrap()),
            's' => {
                let s = args.next().unwrap().as_str().unwrap_or("(null)");
                for ch in s.chars() {
                    consputc(ch as i32);
                }
            }
            '%' => consputc('%' as i32),
            c => {
                consputc('%' as i32);
                consputc(c as i32);
            }
        }
    }

    if locking {
        unsafe { CONS.lock.release() };
    }
}

pub fn panic(s: &str) {
    unsafe {
        cli();
        CONS.locking = false;
        cprintf("cpu{}: panic: ", cpu().id);
        cprintf(s);
        cprintf("\n");
        let mut pcs = [0usize; 10];
        getcallerpcs(&s as *const _ as *const u8, &mut pcs);
        for pc in &pcs {
            cprintf(" {:p}", pc);
        }
        PANICKED = true;
        loop {}
    }
}

const BACKSPACE: i32 = 0x100;
const CRTPORT: u16 = 0x3d4;
static mut CRT: *mut u16 = P2V(0xb8000) as *mut u16;

fn cgaputc(c: i32) {
    let mut pos;

    // Cursor position: col + 80*row.
    outb(CRTPORT, 14);
    pos = (inb(CRTPORT + 1) as i32) << 8;
    outb(CRTPORT, 15);
    pos |= inb(CRTPORT + 1) as i32;

    if c == '\n' as i32 {
        pos += 80 - pos % 80;
    } else if c == BACKSPACE {
        if pos > 0 {
            pos -= 1;
        }
    } else {
        unsafe {
            *CRT.add(pos as usize) = (c as u16 & 0xff) | 0x0700;
        }
        pos += 1;
    }

    if pos / 80 >= 24 {
        unsafe {
            ptr::copy(CRT.add(80), CRT, 23 * 80);
            ptr::write_bytes(CRT.add(pos as usize), 0, 24 * 80 - pos as usize);
        }
        pos -= 80;
    }

    outb(CRTPORT, 14);
    outb(CRTPORT + 1, (pos >> 8) as u8);
    outb(CRTPORT, 15);
    outb(CRTPORT + 1, pos as u8);
    unsafe {
        *CRT.add(pos as usize) = ' ' as u16 | 0x0700;
    }
}

pub fn consputc(c: i32) {
    if unsafe { PANICKED } {
        cli();
        loop {}
    }

    if c == BACKSPACE {
        uartputc('\x08');
        uartputc(' ');
        uartputc('\x08');
    } else {
        uartputc(c as u8);
    }
    cgaputc(c);
}

const INPUT_BUF: usize = 128;

struct Input {
    lock: Spinlock,
    buf: [u8; INPUT_BUF],
    r: u32, // Read index
    w: u32, // Write index
    e: u32, // Edit index
}

static mut INPUT: Input = Input {
    lock: Spinlock::new("input"),
    buf: [0; INPUT_BUF],
    r: 0,
    w: 0,
    e: 0,
};

const fn C(x: u8) -> u8 {
    x - b'@'
}

pub fn consoleintr(getc: fn() -> i32) {
    let mut c;

    unsafe {
        INPUT.lock.acquire();
        while {
            c = getc();
            c >= 0
        } {
            match c as u8 {
                C(b'Z') => lidt(0, 0),
                C(b'P') => procdump(),
                C(b'U') => {
                    while INPUT.e != INPUT.w && INPUT.buf[(INPUT.e - 1) as usize % INPUT_BUF] != b'\n' {
                        INPUT.e -= 1;
                        consputc(BACKSPACE);
                    }
                }
                C(b'H') | 0x7f => {
                    if INPUT.e != INPUT.w {
                        INPUT.e -= 1;
                        consputc(BACKSPACE);
                    }
                }
                _ => {
                    if c != 0 && (INPUT.e - INPUT.r) < INPUT_BUF as u32 {
                        let c = if c == '\r' as i32 { '\n' as i32 } else { c };
                        INPUT.buf[INPUT.e as usize % INPUT_BUF] = c as u8;
                        INPUT.e += 1;
                        consputc(c);
                        if c == '\n' as i32 || c == C(b'D') as i32 || INPUT.e == INPUT.r + INPUT_BUF as u32 {
                            INPUT.w = INPUT.e;
                            wakeup(&INPUT.r as *const _ as *mut u8);
                        }
                    }
                }
            }
        }
        INPUT.lock.release();
    }
}

pub fn consoleread(ip: &mut Inode, dst: &mut [u8], n: usize) -> i32 {
    let mut target = n;
    let mut c;

    iunlock(ip);
    unsafe {
        INPUT.lock.acquire();
        while target > 0 {
            while INPUT.r == INPUT.w {
                if proc().killed {
                    INPUT.lock.release();
                    ilock(ip);
                    return -1;
                }
                sleep(&INPUT.r as *const _ as *mut u8, &INPUT.lock);
            }
            c = INPUT.buf[INPUT.r as usize % INPUT_BUF];
            INPUT.r += 1;
            if c == C(b'D') {
                if target < n {
                    INPUT.r -= 1;
                }
                break;
            }
            dst[n - target] = c;
            target -= 1;
            if c == b'\n' {
                break;
            }
        }
        INPUT.lock.release();
    }
    ilock(ip);

    (n - target) as i32
}

pub fn consolewrite(ip: &mut Inode, buf: &[u8], n: usize) -> i32 {
    iunlock(ip);
    unsafe {
        CONS.lock.acquire();
        for &c in buf.iter().take(n) {
            consputc(c as i32);
        }
        CONS.lock.release();
    }
    ilock(ip);

    n as i32
}

pub fn consoleinit() {
    unsafe {
        CONS.lock.init("console");
        INPUT.lock.init("input");

        devsw[CONSOLE].write = consolewrite;
        devsw[CONSOLE].read = consoleread;
        CONS.locking = true;

        picenable(IRQ_KBD);
        ioapicenable(IRQ_KBD, 0);
    }
}