use crate::include::types:*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::spinlock::*;
use crate::include::fs::*;
use crate::include::buf::*;

// Simple logging that allows concurrent FS system calls.
//
// A log transaction contains the updates of multiple FS system
// calls. The logging system only commits when there are
// no FS system calls active. Thus there is never
// any reasoning required about whether a commit might
// write an uncommitted system call's updates to disk.
//
// A system call should call begin_op()/end_op() to mark
// its start and end. Usually begin_op() just increments
// the count of in-progress FS system calls and returns.
// But if it thinks the log is close to running out, it
// sleeps until the last outstanding end_op() commits.
//
// The log is a physical re-do log containing disk blocks.
// The on-disk log format:
//   header block, containing block #s for block A, B, C, ...
//   block A
//   block B
//   block C
//   ...
// Log appends are synchronous.

// src/kernel/log.rs

use crate::include::spinlock::*;
use crate::include::param::*;
use crate::include::defs::*;
use crate::include::buf::*;
use crate::include::fs::*;
use crate::include::memlayout::*;
use crate::include::x86::*;
use core::ptr;

const LOGSIZE: usize = 10; // Assuming LOGSIZE is defined somewhere
const BSIZE: usize = 512; // Assuming BSIZE is defined somewhere

// Contents of the header block, used for both the on-disk header block
// and to keep track in memory of logged block# before commit.
#[repr(C)]
struct LogHeader {
    n: i32,
    block: [i32; LOGSIZE],
}

#[repr(C)]
struct Log {
    lock: Spinlock,
    start: i32,
    size: i32,
    outstanding: i32, // how many FS sys calls are executing.
    committing: i32,  // in commit(), please wait.
    dev: i32,
    lh: LogHeader,
}

static mut LOG: Log = Log {
    lock: Spinlock::new("log"),
    start: 0,
    size: 0,
    outstanding: 0,
    committing: 0,
    dev: 0,
    lh: LogHeader {
        n: 0,
        block: [0; LOGSIZE],
    },
};

fn recover_from_log() {
    read_head();
    install_trans(); // if committed, copy from log to disk
    unsafe {
        LOG.lh.n = 0;
    }
    write_head(); // clear the log
}

fn commit() {
    unsafe {
        if LOG.lh.n > 0 {
            write_log();     // Write modified blocks from cache to log
            write_head();    // Write header to disk -- the real commit
            install_trans(); // Now install writes to home locations
            LOG.lh.n = 0;
            write_head();    // Erase the transaction from the log
        }
    }
}

pub fn initlog() {
    if core::mem::size_of::<LogHeader>() >= BSIZE {
        panic!("initlog: too big logheader");
    }

    let mut sb = Superblock::default();
    unsafe {
        LOG.lock.init("log");
        readsb(ROOTDEV, &mut sb);
        LOG.start = sb.size - sb.nlog;
        LOG.size = sb.nlog;
        LOG.dev = ROOTDEV;
    }
    recover_from_log();
}

// Copy committed blocks from log to their home location
fn install_trans() {
    unsafe {
        for tail in 0..LOG.lh.n {
            let lbuf = bread(LOG.dev, LOG.start + tail + 1); // read log block
            let dbuf = bread(LOG.dev, LOG.lh.block[tail as usize]); // read dst
            ptr::copy_nonoverlapping(lbuf.data.as_ptr(), dbuf.data.as_mut_ptr(), BSIZE);
            bwrite(dbuf);  // write dst to disk
            brelse(lbuf);
            brelse(dbuf);
        }
    }
}

// Read the log header from disk into the in-memory log header
fn read_head() {
    unsafe {
        let buf = bread(LOG.dev, LOG.start);
        let lh = &*(buf.data.as_ptr() as *const LogHeader);
        LOG.lh.n = lh.n;
        for i in 0..LOG.lh.n {
            LOG.lh.block[i as usize] = lh.block[i as usize];
        }
        brelse(buf);
    }
}

// Write in-memory log header to disk.
// This is the true point at which the
// current transaction commits.
fn write_head() {
    unsafe {
        let buf = bread(LOG.dev, LOG.start);
        let hb = &mut *(buf.data.as_mut_ptr() as *mut LogHeader);
        hb.n = LOG.lh.n;
        for i in 0..LOG.lh.n {
            hb.block[i as usize] = LOG.lh.block[i as usize];
        }
        bwrite(buf);
        brelse(buf);
    }
}

// called at the start of each FS system call.
pub fn begin_op() {
    unsafe {
        LOG.lock.acquire();
        loop {
            if LOG.committing != 0 {
                sleep(&LOG as *const _ as usize, &LOG.lock);
            } else if LOG.lh.n + (LOG.outstanding + 1) * MAXOPBLOCKS > LOGSIZE as i32 {
                // this op might exhaust log space; wait for commit.
                sleep(&LOG as *const _ as usize, &LOG.lock);
            } else {
                LOG.outstanding += 1;
                LOG.lock.release();
                break;
            }
        }
    }
}

// called at the end of each FS system call.
// commits if this was the last outstanding operation.
pub fn end_op() {
    let mut do_commit = 0;

    unsafe {
        LOG.lock.acquire();
        LOG.outstanding -= 1;
        if LOG.committing != 0 {
            panic!("log.committing");
        }
        if LOG.outstanding == 0 {
            do_commit = 1;
            LOG.committing = 1;
        } else {
            // begin_op() may be waiting for log space.
            wakeup(&LOG as *const _ as usize);
        }
        LOG.lock.release();
    }

    if do_commit != 0 {
        // call commit w/o holding locks, since not allowed
        // to sleep with locks.
        commit();
        unsafe {
            LOG.lock.acquire();
            LOG.committing = 0;
            wakeup(&LOG as *const _ as usize);
            LOG.lock.release();
        }
    }
}

// Copy modified blocks from cache to log.
fn write_log() {
    unsafe {
        for tail in 0..LOG.lh.n {
            let to = bread(LOG.dev, LOG.start + tail + 1); // log block
            let from = bread(LOG.dev, LOG.lh.block[tail as usize]); // cache block
            ptr::copy_nonoverlapping(from.data.as_ptr(), to.data.as_mut_ptr(), BSIZE);
            bwrite(to);  // write the log
            brelse(from);
            brelse(to);
        }
    }
}

// Caller has modified b->data and is done with the buffer.
// Record the block number and pin in the cache with B_DIRTY.
// commit()/write_log() will do the disk write.
//
// log_write() replaces bwrite(); a typical use is:
//   bp = bread(...)
//   modify bp->data[]
//   log_write(bp)
//   brelse(bp)
pub fn log_write(b: &mut Buf) {
    unsafe {
        if LOG.lh.n >= LOGSIZE as i32 || LOG.lh.n >= LOG.size - 1 {
            panic!("too big a transaction");
        }
        if LOG.outstanding < 1 {
            panic!("log_write outside of trans");
        }

        LOG.lock.acquire();
        let mut i = 0;
        for j in 0..LOG.lh.n {
            if LOG.lh.block[j as usize] == b.blockno {
                i = j;
                break;
            }
        }
        LOG.lh.block[i as usize] = b.blockno;
        if i == LOG.lh.n {
            LOG.lh.n += 1;
        }
        b.flags |= B_DIRTY; // prevent eviction
        LOG.lock.release();
    }
}