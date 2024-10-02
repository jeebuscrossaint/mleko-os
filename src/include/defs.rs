// src/include/defs.rs

// Define empty structs
pub struct Buf;
pub struct Context;
pub struct File;
pub struct Inode;
pub struct Pipe;
pub struct Proc;
pub struct Rtcdate;
pub struct Spinlock;
pub struct Stat;
pub struct Superblock;

// Define function signatures using extern "C"
extern "C" {
    // bio.c
    pub fn binit();
    pub fn bread(dev: u32, blockno: u32) -> *mut Buf;
    pub fn brelse(b: *mut Buf);
    pub fn bwrite(b: *mut Buf);

    // console.c
    pub fn consoleinit();
    pub fn cprintf(fmt: *const u8, ...);
    pub fn consoleintr(getc: extern "C" fn() -> i32);
    pub fn panic(s: *const u8) -> !;

    // exec.c
    pub fn exec(path: *const u8, argv: *const *const u8) -> i32;

    // file.c
    pub fn filealloc() -> *mut File;
    pub fn fileclose(f: *mut File);
    pub fn filedup(f: *mut File) -> *mut File;
    pub fn fileinit();
    pub fn fileread(f: *mut File, addr: *mut u8, n: i32) -> i32;
    pub fn filestat(f: *mut File, st: *mut Stat) -> i32;
    pub fn filewrite(f: *mut File, addr: *mut u8, n: i32) -> i32;

    // fs.c
    pub fn readsb(dev: i32, sb: *mut Superblock);
    pub fn dirlink(dp: *mut Inode, name: *const u8, inum: u32) -> i32;
    pub fn dirlookup(dp: *mut Inode, name: *const u8, poff: *mut u32) -> *mut Inode;
    pub fn ialloc(dev: u32, typ: i16) -> *mut Inode;
    pub fn idup(ip: *mut Inode) -> *mut Inode;
    pub fn iinit();
    pub fn ilock(ip: *mut Inode);
    pub fn iput(ip: *mut Inode);
    pub fn iunlock(ip: *mut Inode);
    pub fn iunlockput(ip: *mut Inode);
    pub fn iupdate(ip: *mut Inode);
    pub fn namecmp(s: *const u8, t: *const u8) -> i32;
    pub fn namei(path: *const u8) -> *mut Inode;
    pub fn nameiparent(path: *const u8, name: *mut u8) -> *mut Inode;
    pub fn readi(ip: *mut Inode, dst: *mut u8, off: u32, n: u32) -> i32;
    pub fn stati(ip: *mut Inode, st: *mut Stat);
    pub fn writei(ip: *mut Inode, src: *mut u8, off: u32, n: u32) -> i32;

    // ide.c
    pub fn ideinit();
    pub fn ideintr();
    pub fn iderw(b: *mut Buf);

    // ioapic.c
    pub fn ioapicenable(irq: i32, cpu: i32);
    pub static mut ioapicid: u8;
    pub fn ioapicinit();

    // kalloc.c
    pub fn kalloc() -> *mut u8;
    pub fn kfree(ptr: *mut u8);
    pub fn kinit1(vstart: *mut u8, vend: *mut u8);
    pub fn kinit2(vstart: *mut u8, vend: *mut u8);

    // kbd.c
    pub fn kbdintr();

    // lapic.c
    pub fn cmostime(r: *mut Rtcdate);
    pub fn cpunum() -> i32;
    pub static mut lapic: *mut u32;
    pub fn lapiceoi();
    pub fn lapicinit();
    pub fn lapicstartap(apicid: u8, addr: u32);
    pub fn microdelay(us: i32);

    // log.c
    pub fn initlog();
    pub fn log_write(b: *mut Buf);
    pub fn begin_op();
    pub fn end_op();

    // mp.c
    pub static mut ismp: i32;
    pub fn mpbcpu() -> i32;
    pub fn mpinit();
    pub fn mpstartthem();

    // apic.c
    pub fn acpiinit() -> i32;

    // cpuid.c
    pub fn cpuidinit();

    // picirq.c
    pub fn picenable(irq: i32);
    pub fn picinit();

    // pipe.c
    pub fn pipealloc(f0: *mut *mut File, f1: *mut *mut File) -> i32;
    pub fn pipeclose(pi: *mut Pipe, writable: i32);
    pub fn piperead(pi: *mut Pipe, addr: *mut u8, n: i32) -> i32;
    pub fn pipewrite(pi: *mut Pipe, addr: *mut u8, n: i32) -> i32;

    // proc.c
    pub fn copyproc(p: *mut Proc) -> *mut Proc;
    pub fn exit();
    pub fn fork() -> i32;
    pub fn growproc(n: i32) -> i32;
    pub fn kill(pid: i32) -> i32;
    pub fn pinit();
    pub fn procdump();
    pub fn scheduler() -> !;
    pub fn sched();
    pub fn sleep(chan: *mut u8, lk: *mut Spinlock);
    pub fn userinit();
    pub fn wait() -> i32;
    pub fn wakeup(chan: *mut u8);
    pub fn yield();

    // swtch.S
    pub fn swtch(old: *mut *mut Context, new: *mut Context);

    // spinlock.c
    pub fn acquire(lk: *mut Spinlock);
    pub fn getcallerpcs(v: *mut u8, pcs: *mut u32);
    pub fn getstackpcs(pcs: *mut u32, max: u32);
    pub fn holding(lk: *mut Spinlock) -> i32;
    pub fn initlock(lk: *mut Spinlock, name: *const u8);
    pub fn release(lk: *mut Spinlock);
    pub fn pushcli();
    pub fn popcli();

    // string.c
    pub fn memcmp(v1: *const u8, v2: *const u8, n: u32) -> i32;
    pub fn memmove(dst: *mut u8, src: *const u8, n: u32) -> *mut u8;
    pub fn memset(dst: *mut u8, c: i32, n: u32) -> *mut u8;
    pub fn safestrcpy(s: *mut u8, t: *const u8, n: i32) -> *mut u8;
    pub fn strlen(s: *const u8) -> i32;
    pub fn strncmp(s1: *const u8, s2: *const u8, n: u32) -> i32;
    pub fn strncpy(s: *mut u8, t: *const u8, n: i32) -> *mut u8;

    // syscall.c
    pub fn argint(n: i32, ip: *mut i32) -> i32;
    pub fn argptr(n: i32, pp: *mut *mut u8, size: i32) -> i32;
    pub fn argstr(n: i32, pp: *mut *mut u8) -> i32;
    pub fn arguintp(n: i32, pp: *mut u32) -> i32;
    pub fn fetchuintp(uva: u32, pp: *mut u32) -> i32;
    pub fn fetchstr(uva: u32, pp: *mut *mut u8) -> i32;
    pub fn syscall();

    // timer.c
    pub fn timerinit();

    // trap.c
    pub fn idtinit();
    pub static mut ticks: u32;
    pub fn tvinit();
    pub static mut tickslock: Spinlock;

    // uart.c
    pub fn uartearlyinit();
    pub fn uartinit();
    pub fn uartintr();
    pub fn uartputc(c: i32);

    // vm.c
    pub fn seginit();
    pub fn kvmalloc();
    pub fn vmenable();
    pub fn setupkvm() -> *mut u32;
    pub fn uva2ka(pgdir: *mut u32, uva: *mut u8) -> *mut u8;
    pub fn allocuvm(pgdir: *mut u32, oldsz: u32, newsz: u32) -> i32;
    pub fn deallocuvm(pgdir: *mut u32, oldsz: u32, newsz: u32) -> i32;
    pub fn freevm(pgdir: *mut u32);
    pub fn inituvm(pgdir: *mut u32, init: *mut u8, sz: u32);
    pub fn loaduvm(pgdir: *mut u32, addr: *mut u8, ip: *mut Inode, offset: u32, sz: u32) -> i32;
    pub fn copyuvm(pgdir: *mut u32, sz: u32) -> *mut u32;
    pub fn switchuvm(p: *mut Proc);
    pub fn switchkvm();
    pub fn copyout(pgdir: *mut u32, va: u32, p: *mut u8, len: u32) -> i32;
    pub fn clearpteu(pgdir: *mut u32, uva: *mut u8);
}

// Number of elements in fixed-size array
pub fn nelem<T>(array: &[T]) -> usize {
    array.len()
}