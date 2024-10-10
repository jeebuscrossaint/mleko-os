// src/kernel/ide.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::memlayout::*;
use crate::include::mmu::*;
use crate::include::proc::*;
use crate::include::x86::*;
use crate::include::traps::*;
use crate::include::spinlock::*;
use crate::include::fs::*;
use crate::include::buf::*;

// Constants
const SECTOR_SIZE: usize = 512;
const IDE_BSY: u8 = 0x80;
const IDE_DRDY: u8 = 0x40;
const IDE_DF: u8 = 0x20;
const IDE_ERR: u8 = 0x01;

const IDE_CMD_READ: u8 = 0x20;
const IDE_CMD_WRITE: u8 = 0x30;

// idequeue points to the buf now being read/written to the disk.
// idequeue->qnext points to the next buf to be processed.
// You must hold idelock while manipulating queue.

static mut IDELOCK: Spinlock = Spinlock::new("idelock");
static mut IDEQUEUE: Option<&'static mut Buf> = None;

static mut HAVEDISK1: i32 = 0;

fn idestart(buf: &mut Buf);

// Wait for IDE disk to become ready.
fn idewait(checkerr: bool) -> i32 {
    let mut r: i32;

    while {
        r = inb(0x1f7) as i32;
        (r & (IDE_BSY | IDE_DRDY) as i32) != IDE_DRDY as i32
    } {}

    if checkerr && (r & (IDE_DF | IDE_ERR) as i32) != 0 {
        return -1;
    }
    0
}

// Initialize the IDE disk
pub fn ideinit() {
    let mut i: i32;

    unsafe {
        IDELOCK.init("ide");
    }
    picenable(IRQ_IDE);
    ioapicenable(IRQ_IDE, ncpu - 1);
    idewait(false);

    // Check if disk 1 is present
    outb(0x1f6, 0xe0 | (1 << 4));
    for i in 0..1000 {
        if inb(0x1f7) != 0 {
            unsafe {
                HAVEDISK1 = 1;
            }
            break;
        }
    }

    // Switch back to disk 0.
    outb(0x1f6, 0xe0 | (0 << 4));
}

// Start the request for b. Caller must hold idelock.
fn idestart(b: &mut Buf) {
    if b.is_null() {
        panic!("idestart");
    }
    if b.blockno >= FSSIZE {
        panic!("incorrect blockno");
    }
    let sector_per_block = BSIZE / SECTOR_SIZE;
    let sector = b.blockno * sector_per_block;

    if sector_per_block > 7 {
        panic!("idestart");
    }

    idewait(false);
    outb(0x3f6, 0); // generate interrupt
    outb(0x1f2, sector_per_block as u8); // number of sectors
    outb(0x1f3, (sector & 0xff) as u8);
    outb(0x1f4, ((sector >> 8) & 0xff) as u8);
    outb(0x1f5, ((sector >> 16) & 0xff) as u8);
    outb(0x1f6, 0xe0 | ((b.dev & 1) << 4) | ((sector >> 24) & 0x0f) as u8);
    if b.flags & B_DIRTY != 0 {
        outb(0x1f7, IDE_CMD_WRITE);
        unsafe {
            outsl(0x1f0, b.data.as_ptr() as *const u32, BSIZE / 4);
        }
    } else {
        outb(0x1f7, IDE_CMD_READ);
    }
}

// Interrupt handler.
pub fn ideintr() {
    let mut b: Option<&mut Buf>;

    // First queued buffer is the active request.
    unsafe {
        IDELOCK.acquire();
        b = IDEQUEUE.take();
        if b.is_none() {
            IDELOCK.release();
            // cprintf("spurious IDE interrupt\n");
            return;
        }
        IDEQUEUE = b.as_mut().unwrap().qnext.take();

        // Read data if needed.
        if b.as_ref().unwrap().flags & B_DIRTY == 0 && idewait(true) >= 0 {
            unsafe {
                insl(0x1f0, b.as_mut().unwrap().data.as_mut_ptr() as *mut u32, BSIZE / 4);
            }
        }

        // Wake process waiting for this buf.
        b.as_mut().unwrap().flags |= B_VALID;
        b.as_mut().unwrap().flags &= !B_DIRTY;
        wakeup(b.as_mut().unwrap() as *mut _ as usize);

        // Start disk on next buf in queue.
        if IDEQUEUE.is_some() {
            idestart(IDEQUEUE.as_mut().unwrap());
        }

        IDELOCK.release();
    }
}

// Sync buf with disk.
// If B_DIRTY is set, write buf to disk, clear B_DIRTY, set B_VALID.
// Else if B_VALID is not set, read buf from disk, set B_VALID.
pub fn iderw(b: &mut Buf) {
    if b.flags & B_BUSY == 0 {
        panic!("iderw: buf not busy");
    }
    if b.flags & (B_VALID | B_DIRTY) == B_VALID {
        panic!("iderw: nothing to do");
    }
    if b.dev != 0 && unsafe { HAVEDISK1 } == 0 {
        panic!("iderw: ide disk 1 not present");
    }

    unsafe {
        IDELOCK.acquire();

        // Append b to idequeue.
        b.qnext = None;
        let mut pp = &mut IDEQUEUE;
        while let Some(ref mut buf) = pp {
            pp = &mut buf.qnext;
        }
        *pp = Some(b);

        // Start disk if necessary.
        if IDEQUEUE.as_ref() == Some(&b) {
            idestart(b);
        }

        // Wait for request to finish.
        while b.flags & (B_VALID | B_DIRTY) != B_VALID {
            sleep(b as *mut _ as usize, &IDELOCK);
        }

        IDELOCK.release();
    }
}