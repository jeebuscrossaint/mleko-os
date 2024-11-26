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

// find inode number in inum on device dev and return the in memory copy. doesnt lock inode and does not read from disk.
fn iget(dev: u32, inum: u32) -> Option<&'static mut Inode> {
    let mut empty: Option<&mut Inode> = None;

    unsafe {
        ICACHE.lock.acquire();

        // Is the inode already cached?
        for ip in ICACHE.inode.iter_mut() {
            if ip.ref_count > 0 && ip.dev == dev && ip.inum == inum {
                ip.ref_count += 1;
                ICACHE.lock.release();
                return Some(ip);
            }
            if empty.is_none() && ip.ref_count == 0 {
                // Remember empty slot.
                empty = Some(ip);
            }
        }

        // Recycle an inode cache entry.
        if empty.is_none() {
            panic!("iget: no inodes");
        }

        let ip = empty.unwrap();
        ip.dev = dev;
        ip.inum = inum;
        ip.ref_count = 1;
        ip.flags = 0;
        ICACHE.lock.release();

        Some(ip)
    }
}

// increment reference count for ip
// Returns ip to enable ip = idup(ip1) idiom.

pub fn idup(ip: &mut Inode) -> &mut Inode {
    unsafe {
        ICACHE.lock.acquire();
        ip.ref_count += 1;
        ICACHE.lock.release();
    }
    ip
}

// lock a given inode
// reads inode from disk if necessary

pub fn ilock(ip: &mut Inode) {
    if ip.is_null() || ip.ref_count < 1 {
        panic!("ilock");
    }

    unsafe {
        ICACHE.lock.acquire();
        while ip.flags & I_BUSY != 0 {
            sleep(ip as *mut _ as usize, &ICACHE.lock);
        }
        ip.flags |= I_BUSY;
        ICACHE.lock.release();

        if ip.flags & I_VALID == 0 {
            let mut bp = bread(ip.dev as i32, IBLOCK(ip.inum, sb.ninodes));
            let dip = (bp.data.as_mut_ptr() as *mut Dinode).add((ip.inum % IPB) as usize);

            ip.type_ = (*dip).type_;
            ip.major = (*dip).major;
            ip.minor = (*dip).minor;
            ip.nlink = (*dip).nlink;
            ip.size = (*dip).size;
            ip.ownerid = (*dip).ownerid;
            ip.groupid = (*dip).groupid;
            ip.mode = (*dip).mode;
            core::ptr::copy_nonoverlapping((*dip).addrs.as_ptr(), ip.addrs.as_mut_ptr(), ip.addrs.len());

            brelse(bp);
            ip.flags |= I_VALID;
            if ip.type_ == 0 {
                panic!("ilock: no type");
            }
        }
    }
}

// unlock a given inode

pub fn iunlock(ip: &mut Inode) {
    if ip.is_null() || ip.flags & I_BUSY == 0 || ip.ref_count < 1 {
        panic!("iunlock");
    }

    unsafe {
        ICACHE.lock.acquire();
        ip.flags &= !I_BUSY;
        wakeup(ip as *mut _ as usize);
        ICACHE.lock.release();
    }
}

// Drop a reference to an in-memory inode.
// If that was the last reference, the inode cache entry can
// be recycled.
// If that was the last reference and the inode has no links
// to it, free the inode (and its content) on disk.
// All calls to iput() must be inside a transaction in
// case it has to free the inode.

pub fn iput(ip: &mut Inode) {
    unsafe {
        ICACHE.lock.acquire();
        if ip.ref_count == 1 && (ip.flags & I_VALID != 0) && ip.nlink == 0 {
            // inode has no links and no other references: truncate and free.
            if ip.flags & I_BUSY != 0 {
                panic!("iput busy");
            }
            ip.flags |= I_BUSY;
            ICACHE.lock.release();
            itrunc(ip);
            ip.type_ = 0;
            iupdate(ip);
            ICACHE.lock.acquire();
            ip.flags = 0;
            wakeup(ip as *mut _ as usize);
        }
        ip.ref_count -= 1;
        ICACHE.lock.release();
    }
}

// Unlock and put an inode
pub fn iunlockput(ip: &mut Inode) {
    iunlock(ip);
    iput(ip);
}

// Inode content
//
// The content (data) associated with each inode is stored
// in blocks on the disk. The first NDIRECT block numbers
// are listed in ip->addrs[].  The next NINDIRECT blocks are 
// listed in block ip->addrs[NDIRECT].

// Return the disk block address of the nth block in inode ip.
// If there is no such block, bmap allocates one.

pub fn bmap(ip: &mut Inode, bn: u32) -> u32 {
    let mut addr: u32;
    let mut a: *mut u32;
    let mut bp: Option<&mut Buf>;

    if bn < NDIRECT as u32 {
        addr = ip.addrs[bn as usize];
        if addr == 0 {
            ip.addrs[bn as usize] = balloc(ip.dev);
            addr = ip.addrs[bn as usize];
        }
        return addr;
    }
    let mut bn = bn - NDIRECT as u32;

    if bn < NINDIRECT as u32 {
        // Load indirect block, allocating if necessary.
        addr = ip.addrs[NDIRECT];
        if addr == 0 {
            ip.addrs[NDIRECT] = balloc(ip.dev);
            addr = ip.addrs[NDIRECT];
        }
        bp = Some(bread(ip.dev as i32, addr));
        a = bp.as_mut().unwrap().data.as_mut_ptr() as *mut u32;
        addr = unsafe { *a.add(bn as usize) };
        if addr == 0 {
            unsafe { *a.add(bn as usize) = balloc(ip.dev) };
            addr = unsafe { *a.add(bn as usize) };
            log_write(bp.as_mut().unwrap());
        }
        brelse(bp.take().unwrap());
        return addr;
    }

    panic!("bmap: out of range");
}

// Truncate inode (discard contents).
// Only called when the inode has no links
// to it (no directory entries referring to it)
// and has no in-memory reference to it (is
// not an open file or current directory).

fn itrunc(ip: &mut Inode) {
    let mut i: usize;
    let mut j: usize;
    let mut bp: Option<&mut Buf>;
    let mut a: *mut u32;

    for i in 0..NDIRECT {
        if ip.addrs[i] != 0 {
            bfree(ip.dev, ip.addrs[i]);
            ip.addrs[i] = 0;
        }
    }

    if ip.addrs[NDIRECT] != 0 {
        bp = Some(bread(ip.dev as i32, ip.addrs[NDIRECT]));
        a = bp.as_mut().unwrap().data.as_mut_ptr() as *mut u32;
        for j in 0..NINDIRECT {
            if unsafe { *a.add(j) } != 0 {
                bfree(ip.dev, unsafe { *a.add(j) });
            }
        }
        brelse(bp.take().unwrap());
        bfree(ip.dev, ip.addrs[NDIRECT]);
        ip.addrs[NDIRECT] = 0;
    }

    ip.size = 0;
    iupdate(ip);
}

// copy stat information from inode
pub fn stati(ip: &Inode, st: &mut Stat) {
    st.dev = ip.dev;
    st.ino = ip.inum;
    st.type_ = ip.type_;
    st.nlink = ip.nlink;
    st.size = ip.size;
    st.ownerid = ip.ownerid;
    st.groupid = ip.groupid;
    st.mode = ip.mode;
}

// read data form inode
pub fn readi(ip: &mut Inode, dst: &mut [u8], mut off: u32, mut n: u32) -> i32 {
    let mut tot: u32;
    let mut m: u32;
    let mut bp: Option<&mut Buf>;

    if ip.type_ == T_DEV {
        if ip.major < 0 || ip.major >= NDEV as i16 || devsw[ip.major as usize].read.is_none() {
            return -1;
        }
        return devsw[ip.major as usize].read.unwrap()(ip, dst, n);
    }

    if off > ip.size || off + n < off {
        return -1;
    }
    if off + n > ip.size {
        n = ip.size - off;
    }

    tot = 0;
    while tot < n {
        bp = Some(bread(ip.dev as i32, bmap(ip, off / BSIZE)));
        m = min(n - tot, BSIZE - (off % BSIZE));
        unsafe {
            core::ptr::copy_nonoverlapping(
                bp.as_ref().unwrap().data.as_ptr().add((off % BSIZE) as usize),
                dst.as_mut_ptr().add(tot as usize),
                m as usize,
            );
        }
        brelse(bp.take().unwrap());
        tot += m;
        off += m;
    }
    n as i32
}

// write data to inode (:O)
pub fn writei(ip: &mut Inode, src: &[u8], mut off: u32, mut n: u32) -> i32 {
    let mut tot: u32;
    let mut m: u32;
    let mut bp: Option<&mut Buf>;

    if ip.type_ == T_DEV {
        if ip.major < 0 || ip.major >= NDEV as i16 || devsw[ip.major as usize].write.is_none() {
            return -1;
        }
        return devsw[ip.major as usize].write.unwrap()(ip, src, n);
    }

    if off > ip.size || off + n < off {
        return -1;
    }
    if off + n > (MAXFILE * BSIZE) as u32 {
        return -1;
    }

    tot = 0;
    while tot < n {
        bp = Some(bread(ip.dev as i32, bmap(ip, off / BSIZE)));
        m = min(n - tot, BSIZE - (off % BSIZE));
        unsafe {
            core::ptr::copy_nonoverlapping(
                src.as_ptr().add(tot as usize),
                bp.as_mut().unwrap().data.as_mut_ptr().add((off % BSIZE) as usize),
                m as usize,
            );
        }
        log_write(bp.as_mut().unwrap());
        brelse(bp.take().unwrap());
        tot += m;
        off += m;
    }

    if n > 0 && off > ip.size {
        ip.size = off;
        iupdate(ip);
    }
    n as i32
}

// Directories implementation
pub fn namecmp(s: %str, t: &str) -> i32 {
    s.chars()
        .zip(t.chars())
        .take(DIRSIZ)
        .find(|&(sc, tc)| sc != tc)
        .map_or(0, |(sc, tc)| sc as i32 - tc as i32)
}

// look for dirs in dirs
// if found set *poff to byte offset in dir
pub fn dirlookup(dp: &mut Inode, name: &str, poff: Option<&mut u32>) -> Option<&'static mut Inode> {
    let mut off: u32;
    let mut inum: u32;
    let mut de: Dirent = Dirent::default();

    if dp.type_ != T_DIR {
        panic!("dirlookup not DIR");
    }

    off = 0;
    while off < dp.size {
        if readi(dp, unsafe { core::slice::from_raw_parts_mut(&mut de as *mut _ as *mut u8, core::mem::size_of::<Dirent>()) }, off, core::mem::size_of::<Dirent>() as u32) != core::mem::size_of::<Dirent>() as i32 {
            panic!("dirlink read");
        }
        if de.inum == 0 {
            off += core::mem::size_of::<Dirent>() as u32;
            continue;
        }
        if namecmp(name, &de.name) == 0 {
            // entry matches path element
            if let Some(poff) = poff {
                *poff = off;
            }
            inum = de.inum;
            return iget(dp.dev, inum);
        }
        off += core::mem::size_of::<Dirent>() as u32;
    }

    None
}

// write a new dir entry (name, inum) into the dir dp

pub fn dirlink(dp: &mut Inode, name: &str, inum: u32) -> i32 {
    let mut off: u32;
    let mut de: Dirent = Dirent::default();
    let mut ip: Option<&mut Inode>;

    // Check that name is not present.
    ip = dirlookup(dp, name, None);
    if ip.is_some() {
        iput(ip.unwrap());
        return -1;
    }

    // Look for an empty dirent.
    off = 0;
    while off < dp.size {
        if readi(dp, unsafe { core::slice::from_raw_parts_mut(&mut de as *mut _ as *mut u8, core::mem::size_of::<Dirent>()) }, off, core::mem::size_of::<Dirent>() as u32) != core::mem::size_of::<Dirent>() as i32 {
            panic!("dirlink read");
        }
        if de.inum == 0 {
            break;
        }
        off += core::mem::size_of::<Dirent>() as u32;
    }

    de.name[..name.len()].copy_from_slice(name.as_bytes());
    de.inum = inum;
    if writei(dp, unsafe { core::slice::from_raw_parts(&de as *const _ as *const u8, core::mem::size_of::<Dirent>()) }, off, core::mem::size_of::<Dirent>() as u32) != core::mem::size_of::<Dirent>() as i32 {
        panic!("dirlink");
    }

    0
}

// Paths

// Copy the next path element from path into name.
// Return a pointer to the element following the copied one.
// The returned path has no leading slashes,
// so the caller can check *path=='\0' to see if the name is the last one.
// If no name to remove, return 0.
//
// Examples:
//   skipelem("a/bb/c", name) = "bb/c", setting name = "a"
//   skipelem("///a//bb", name) = "bb", setting name = "a"
//   skipelem("a", name) = "", setting name = "a"
//   skipelem("", name) = skipelem("////", name) = 0
//

pub fn skipelem<'a>(path: &'a str, name: &mut [u8]) -> Option<&'a str> {
    let mut s = path.trim_start_matches('/');
    if s.is_empty() {
        return None;
    }

    let end = s.find('/').unwrap_or_else(|| s.len());
    let len = end.min(DIRSIZ);

    name[..len].copy_from_slice(&s.as_bytes()[..len]);
    if len < DIRSIZ {
        name[len] = 0;
    }

    s = &s[end..].trim_start_matches('/');
    Some(s)
}

// Look up and return the inode for a path name.
// If parent != 0, return the inode for the parent and copy the final
// path element into name, which must have room for DIRSIZ bytes.
// Must be called inside a transaction since it calls iput().

fn namex(path: &str, nameiparent: bool, name: &mut [u8]) -> Option<&'static mut Inode> {
    let mut ip: &mut Inode;
    let mut next: Option<&mut Inode>;

    if path.starts_with('/') {
        ip = iget(ROOTDEV, ROOTINO)?;
    } else {
        ip = idup(unsafe { &mut *proc().cwd });
    }

    let mut path = path;
    while let Some(next_path) = skipelem(path, name) {
        ilock(ip);
        if ip.type_ != T_DIR {
            iunlockput(ip);
            return None;
        }
        if nameiparent && next_path.is_empty() {
            // Stop one level early.
            iunlock(ip);
            return Some(ip);
        }
        next = dirlookup(ip, std::str::from_utf8(name).unwrap(), None);
        if next.is_none() {
            iunlockput(ip);
            return None;
        }
        iunlockput(ip);
        ip = next.unwrap();
        path = next_path;
    }

    if nameiparent {
        iput(ip);
        return None;
    }
    Some(ip)
}

// Look up and return the inode for a path name.
pub fn namei(path: &str) -> Option<&'static mut Inode> {
    let mut name = [0u8; DIRSIZ];
    namex(path, false, &mut name)
}

// Look up and return the inode for the parent of the last element in the path.
pub fn nameiparent(path: &str, name: &mut [u8]) -> Option<&'static mut Inode> {
    namex(path, true, name)
}