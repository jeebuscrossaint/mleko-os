// src/include/bio.rs

use crate::include::buf::*;
use crate::include::defs::*;
use crate::include::fs::*;
use crate::include::param::*;
use crate::include::spinlock::*;
use core::ptr;

struct BCache {
    lock: Spinlock,
    buf: [Buf; NBUF],
    head: Buf,
}

static mut BCACHE: BCache = BCache {
    lock: Spinlock::new("bcache"),
    buf: [Buf::new(); NBUF],
    head: Buf::new(),
};

pub fn binit() {
    let bcache = unsafe { &mut BCACHE };
    let head = &mut bcache.head;

    initlock(&mut bcache.lock, "bcache");

    // Create linked list of buffers
    head.prev = head;
    head.next = head;
    for b in bcache.buf.iter_mut() {
        b.next = head.next;
        b.prev = head;
        b.dev = -1;
        unsafe {
            (*head.next).prev = b;
        }
        head.next = b;
    }
}

// Look through buffer cache for block on device dev.
// If not found, allocate a buffer.
// In either case, return B_BUSY buffer.
fn bget(dev: u32, blockno: u32) -> &'static mut Buf {
    let bcache = unsafe { &mut BCACHE };

    acquire(&mut bcache.lock);

    loop {
        // Is the block already cached?
        let mut b = bcache.head.next;
        while b != &mut bcache.head {
            if b.dev == dev && b.blockno == blockno {
                if (b.flags & B_BUSY) == 0 {
                    b.flags |= B_BUSY;
                    release(&mut bcache.lock);
                    return b;
                }
                sleep(b as *mut _, &mut bcache.lock);
                continue;
            }
            b = b.next;
        }

        // Not cached; recycle some non-busy and clean buffer.
        // "clean" because B_DIRTY and !B_BUSY means log.c
        // hasn't yet committed the changes to the buffer.
        let mut b = bcache.head.prev;
        while b != &mut bcache.head {
            if (b.flags & B_BUSY) == 0 && (b.flags & B_DIRTY) == 0 {
                b.dev = dev;
                b.blockno = blockno;
                b.flags = B_BUSY;
                release(&mut bcache.lock);
                return b;
            }
            b = b.prev;
        }
        panic!("bget: no buffers");
    }
}

// Return a B_BUSY buf with the contents of the indicated block.
pub fn bread(dev: u32, blockno: u32) -> &'static mut Buf {
    let b = bget(dev, blockno);
    if (b.flags & B_VALID) == 0 {
        iderw(b);
    }
    b
}

// Write b's contents to disk. Must be B_BUSY.
pub fn bwrite(b: &mut Buf) {
    if (b.flags & B_BUSY) == 0 {
        panic!("bwrite");
    }
    b.flags |= B_DIRTY;
    iderw(b);
}

// Release a B_BUSY buffer.
// Move to the head of the MRU list.
pub fn brelse(b: &mut Buf) {
    if (b.flags & B_BUSY) == 0 {
        panic!("brelse");
    }

    let bcache = unsafe { &mut BCACHE };

    acquire(&mut bcache.lock);

    unsafe {
        (*b.next).prev = b.prev;
        (*b.prev).next = b.next;
    }
    b.next = bcache.head.next;
    b.prev = &mut bcache.head;
    unsafe {
        (*bcache.head.next).prev = b;
    }
    bcache.head.next = b;

    b.flags &= !B_BUSY;
    wakeup(b as *mut _);

    release(&mut bcache.lock);
}