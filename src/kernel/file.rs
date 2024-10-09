// src/kernel/file.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::fs::*;
use crate::include::file::*;
use crate::include::spinlock::*;

static mut DEVS: [Devsw; NDEV] = [Devsw::default(); NDEV];
static mut FTABLE: FTable = FTable {
    lock: Spinlock::new("ftable"),
    file: [File::default(); NFILE],
};

struct FTable {
    lock: Spinlock,
    file: [File; NFILE],
}

pub fn fileinit() {
    unsafe {
        FTABLE.lock.init("ftable");
    }
}

// Allocate a file structure.
pub fn filealloc() -> Option<&'static mut File> {
    unsafe {
        FTABLE.lock.acquire();
        for f in FTABLE.file.iter_mut() {
            if f.ref_count == 0 {
                f.ref_count = 1;
                FTABLE.lock.release();
                return Some(f);
            }
        }
        FTABLE.lock.release();
    }
    None
}

// Increment ref count for file f.
pub fn filedup(f: &mut File) -> &mut File {
    unsafe {
        FTABLE.lock.acquire();
        if f.ref_count < 1 {
            panic!("filedup");
        }
        f.ref_count += 1;
        FTABLE.lock.release();
    }
    f
}

// Close file f. (Decrement ref count, close when reaches 0.)
pub fn fileclose(f: &mut File) {
    let ff: File;

    unsafe {
        FTABLE.lock.acquire();
        if f.ref_count < 1 {
            panic!("fileclose");
        }
        if f.ref_count -= 1; f.ref_count > 0 {
            FTABLE.lock.release();
            return;
        }
        ff = f.clone();
        f.ref_count = 0;
        f.type_ = FD_NONE;
        FTABLE.lock.release();
    }

    if ff.type_ == FD_PIPE {
        pipeclose(ff.pipe.unwrap(), ff.writable);
    } else if ff.type_ == FD_INODE {
        begin_op();
        iput(ff.ip.unwrap());
        end_op();
    }
}

// Get metadata about file f.
pub fn filestat(f: &File, st: &mut Stat) -> i32 {
    if f.type_ == FD_INODE {
        ilock(f.ip.unwrap());
        stati(f.ip.unwrap(), st);
        iunlock(f.ip.unwrap());
        return 0;
    }
    -1
}

// Read from file f.
pub fn fileread(f: &mut File, addr: &mut [u8], n: i32) -> i32 {
    let mut r: i32;

    if f.readable == 0 {
        return -1;
    }
    if f.type_ == FD_PIPE {
        return piperead(f.pipe.unwrap(), addr, n);
    }
    if f.type_ == FD_INODE {
        ilock(f.ip.unwrap());
        r = readi(f.ip.unwrap(), addr, f.off, n);
        if r > 0 {
            f.off += r;
        }
        iunlock(f.ip.unwrap());
        return r;
    }
    panic!("fileread");
}

// Write to file f.
pub fn filewrite(f: &mut File, addr: &[u8], n: i32) -> i32 {
    let mut r: i32;

    if f.writable == 0 {
        return -1;
    }
    if f.type_ == FD_PIPE {
        return pipewrite(f.pipe.unwrap(), addr, n);
    }
    if f.type_ == FD_INODE {
        // write a few blocks at a time to avoid exceeding
        // the maximum log transaction size, including
        // i-node, indirect block, allocation blocks,
        // and 2 blocks of slop for non-aligned writes.
        // this really belongs lower down, since writei()
        // might be writing a device like the console.
        let max = ((LOGSIZE - 1 - 1 - 2) / 2) * 512;
        let mut i = 0;
        while i < n {
            let mut n1 = n - i;
            if n1 > max {
                n1 = max;
            }

            begin_op();
            ilock(f.ip.unwrap());
            r = writei(f.ip.unwrap(), &addr[i as usize..], f.off, n1);
            if r > 0 {
                f.off += r;
            }
            iunlock(f.ip.unwrap());
            end_op();

            if r < 0 {
                break;
            }
            if r != n1 {
                panic!("short filewrite");
            }
            i += r;
        }
        return if i == n { n } else { -1 };
    }
    panic!("filewrite");
}