//src/kernel/fs.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::param::*;
use crate::include::stat::*;
use crate::include::mmu::*;
use crate::include::proc::*;
use crate::include::spinlock::*;
use crate::include::fs::*;
use crate::include::buf::*;
use crate::include::file::*;

// min function
fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

// forward declaration of itrunc
fn itrunc(inode: $mut Inode);

// read a super(man) block theres a star man waiting in the sky hed like to come and meet us but he things he would something something idk

pub fn readsb(dev: i32, sb: $mut Superblock) {
    let bp = bread(dev, 1);
    unsafe {
        core::ptr::copy_nonoverlapping(bp.data.as_ptr(), sb as *mut _ as *mut u8, core::mem::size_of::<Superblock>());
    }
    brelse(bp);
}

// block zeroing function 
pub fn bzero(dev: i32, bno: i32) {
    let bp = bread(dev, bno);
    unsafe {
        core::ptr::write_bytes(bp.data.as_mut_ptr(), 0, BSIZE);
    }
    log_write(bp);
    brelse(bp);
}

// alloc a zeroed disk block
pub fn balloc(dev: u32) -> u32 {
    let mut b: i32;
    let mut bi: i32;
    let mut m: u8;
    let mut bp: Option<&mut Buf> = None;
    let mut sb: Superblock = Superblock::default();

    readsb(dev as i32, &mut sb);
    for b in (0..sb.size).step_by(BPB) {
        bp = Some(bread(dev as i32, BBLOCK(b, sb.ninodes)));
        for bi in 0..BPB {
            if b + bi >= sb.size {
                break;
            }
            m = 1 << (bi % 8);
            if (bp.as_ref().unwrap().data[(bi / 8) as usize] & m) == 0 {
                // Is block free?
                bp.as_mut().unwrap().data[(bi / 8) as usize] |= m; // Mark block in use.
                log_write(bp.as_mut().unwrap());
                brelse(bp.take().unwrap());
                bzero(dev as i32, b + bi);
                return b + bi;
            }
        }
        brelse(bp.take().unwrap());
    }
    panic!("balloc: out of blocks");
}

// free a block
pub fn bfree(dev: i32, b:u32) {
    let mut sb: Superblock = Superblock::default();
    let mut bi: usize;
    let mut m: u8;

    readsb(dev, &mut sb);
    let mut bp = bread(dev, BBLOCK(b, sb.ninodes));
    bi = (b % BPB) as usize;
    m = 1 << (bi % 8);
    if (bp.data[bi/8] & m) == 0 {
        panic!("freeing free block");
    }
    bp.data[bi/8] &= !m;
    log_write(bp);
    brelse(bp);
}

// -- Comments from the original code -- (may be important for anyone interested in understanding the code or the original C implementation)

// Inodes.
//
// An inode describes a single unnamed file.
// The inode disk structure holds metadata: the file's type,
// its size, the number of links referring to it, and the
// list of blocks holding the file's content.
//
// The inodes are laid out sequentially on disk immediately after
// the superblock. Each inode has a number, indicating its
// position on the disk.
//
// The kernel keeps a cache of in-use inodes in memory
// to provide a place for synchronizing access
// to inodes used by multiple processes. The cached
// inodes include book-keeping information that is
// not stored on disk: ip->ref and ip->flags.
//
// An inode and its in-memory represtative go through a
// sequence of states before they can be used by the
// rest of the file system code.
//
// * Allocation: an inode is allocated if its type (on disk)
//   is non-zero. ialloc() allocates, iput() frees if
//   the link count has fallen to zero.
//
// * Referencing in cache: an entry in the inode cache
//   is free if ip->ref is zero. Otherwise ip->ref tracks
//   the number of in-memory pointers to the entry (open
//   files and current directories). iget() to find or
//   create a cache entry and increment its ref, iput()
//   to decrement ref.
//
// * Valid: the information (type, size, &c) in an inode
//   cache entry is only correct when the I_VALID bit
//   is set in ip->flags. ilock() reads the inode from
//   the disk and sets I_VALID, while iput() clears
//   I_VALID if ip->ref has fallen to zero.
//
// * Locked: file system code may only examine and modify
//   the information in an inode and its content if it
//   has first locked the inode. The I_BUSY flag indicates
//   that the inode is locked. ilock() sets I_BUSY,
//   while iunlock clears it.
//
// Thus a typical sequence is:
//   ip = iget(dev, inum)
//   ilock(ip)
//   ... examine and modify ip->xxx ...
//   iunlock(ip)
//   iput(ip)
//
// ilock() is separate from iget() so that system calls can
// get a long-term reference to an inode (as for an open file)
// and only lock it for short periods (e.g., in read()).
// The separation also helps avoid deadlock and races during
// pathname lookup. iget() increments ip->ref so that the inode
// stays cached and pointers to it remain valid.
//
// Many internal file system functions expect the caller to
// have locked the inodes involved; this lets callers create
// multi-step atomic operations.
// --- END COMMENT FROM LINE 90 ---

struct ICache {
    lock: Spinlock,
    inode: [Inode; NINODE],
}

static mut ICACHE: ICache = ICache {
    lock: Spinlock::new("icache"),
    inode: [Inode::default(); NINODE],
};

// init inode cache
pub fn iinit() {
    unsafe {
        ICACHE.lock.init("icache");
    }
}

// forward declaration of iget func
fn iget(dev: u32, inum: u32) -> Option<&'static mut Inode>;

//allocate a new inode for a type on device dev, free inodes have type 0
pub fn ialloc(dev: u32, type_: i16) -> Option<&'static mut Inode> {
    let mut inum: u32;
    let mut bp: Option<&mut Buf>;
    let mut dip: *mut  Dinode;
    let mut sb: Superblock = Superblock::default();

    readsb(dev as i32, &mut sb);

    for inum in 1..sb.ninodes {
        bp = Some(bread(dev as i32, IBLOCK(inum, sb.ninodes)));
        dip = unsafe { (bp.as_mut().unwrap().data.as_mut_ptr() as *mut Dinode).add((inum % IPB) as usize) };
        if unsafe { (*dip).type_ } == 0 {
            // a free inode
            unsafe {
                core::ptr::write_bytes(dip, 0, core::mem::size_of::<Dinode>());
                (*dip).type_ = type_;
            }
            log_write(bp.as_mut().unwrap()); // mark it allocated on the disk
            brelse(bp.take().unwrap());
            return iget(dev, inum);
        }
        brelse(bp.take().unwrap());
    }
    panic!("ialloc: no inodes");
}

// copy modded in memory inode to disk

pub fn iupdate(ip: &mut Inode) {
    let mut bp = bread(ip.dev as i32, IBLOCK(ip.inum, sb.ninodes));
    let dip = unsafe { (bp.data.as_mut_ptr() as *mut Dinode).add((ip.inum % IPB) as usize) };

    unsafe {
        (*dip).type_ = ip.type_;
        (*dip).major = ip.major;
        (*dip).minor = ip.minor;
        (*dip).nlink = ip.nlink;
        (*dip).size = ip.size;
        (*dip).mode = ip.mode;
        core::ptr::copy_nonoverlapping(ip.addrs.as_ptr(), (*dip).addrs.as_mut_ptr(), ip.addrs.len());
    }

    log_write(&mut bp);
    brelse(bp);
}