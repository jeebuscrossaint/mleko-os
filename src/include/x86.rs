// src/x86.rs

#![feature(asm)]

pub type Uchar = u8;
pub type Ushort = u16;
pub type Uint = u32;
pub type Uintp = usize;

#[inline(always)]
pub unsafe fn inb(port: Ushort) -> Uchar {
    let data: Uchar;
    asm!("inb %dx, %al", out("al") data, in("dx") port, options(nostack, nomem));
    data
}

#[inline(always)]
pub unsafe fn insl(port: i32, addr: *mut u8, cnt: i32) {
    asm!("cld; rep insl", in("dx") port, inout("edi") addr, inout("ecx") cnt, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn outb(port: Ushort, data: Uchar) {
    asm!("outb %al, %dx", in("al") data, in("dx") port, options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn outw(port: Ushort, data: Ushort) {
    asm!("outw %ax, %dx", in("ax") data, in("dx") port, options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn outsl(port: i32, addr: *const u8, cnt: i32) {
    asm!("cld; rep outsl", in("dx") port, inout("esi") addr, inout("ecx") cnt, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn stosb(addr: *mut u8, data: i32, cnt: i32) {
    asm!("cld; rep stosb", inout("edi") addr, inout("ecx") cnt, in("al") data, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn stosl(addr: *mut u8, data: i32, cnt: i32) {
    asm!("cld; rep stosl", inout("edi") addr, inout("ecx") cnt, in("eax") data, options(nostack, preserves_flags));
}

#[repr(C)]
pub struct Segdesc;

#[inline(always)]
pub unsafe fn lgdt(p: *const Segdesc, size: i32) {
    let pd: [Ushort; 5] = [
        (size - 1) as Ushort,
        (p as Uintp) as Ushort,
        ((p as Uintp) >> 16) as Ushort,
        #[cfg(target_pointer_width = "64")]
        ((p as Uintp) >> 32) as Ushort,
        #[cfg(target_pointer_width = "64")]
        ((p as Uintp) >> 48) as Ushort,
    ];
    asm!("lgdt ({0})", in(reg) &pd, options(nostack, nomem));
}

#[repr(C)]
pub struct Gatedesc;

#[inline(always)]
pub unsafe fn lidt(p: *const Gatedesc, size: i32) {
    let pd: [Ushort; 5] = [
        (size - 1) as Ushort,
        (p as Uintp) as Ushort,
        ((p as Uintp) >> 16) as Ushort,
        #[cfg(target_pointer_width = "64")]
        ((p as Uintp) >> 32) as Ushort,
        #[cfg(target_pointer_width = "64")]
        ((p as Uintp) >> 48) as Ushort,
    ];
    asm!("lidt ({0})", in(reg) &pd, options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn ltr(sel: Ushort) {
    asm!("ltr {0:x}", in(reg) sel, options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn readeflags() -> Uintp {
    let eflags: Uintp;
    asm!("pushf; pop {0}", out(reg) eflags, options(nostack, nomem));
    eflags
}

#[inline(always)]
pub unsafe fn loadgs(v: Ushort) {
    asm!("mov {0:x}, %gs", in(reg) v, options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn cli() {
    asm!("cli", options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn sti() {
    asm!("sti", options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn hlt() {
    asm!("hlt", options(nostack, nomem));
}

#[inline(always)]
pub unsafe fn xchg(addr: *mut Uint, newval: Uintp) -> Uint {
    let result: Uint;
    asm!("lock xchgl {0}, {1}", inout(reg) newval => result, inout(reg) addr, options(nostack, preserves_flags));
    result
}

#[inline(always)]
pub unsafe fn rcr2() -> Uintp {
    let val: Uintp;
    asm!("mov %cr2, {0}", out(reg) val, options(nostack, nomem));
    val
}

#[inline(always)]
pub unsafe fn lcr3(val: Uintp) {
    asm!("mov {0}, %cr3", in(reg) val, options(nostack, nomem));
}

#[repr(C)]
#[cfg(target_pointer_width = "64")]
pub struct Trapframe {
    pub eax: u64,      // rax
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub trapno: u64,
    pub err: u64,
    pub eip: u64,     // rip
    pub cs: u64,
    pub eflags: u64,  // rflags
    pub esp: u64,     // rsp
    pub ds: u64,      // ss
}

#[repr(C)]
#[cfg(target_pointer_width = "32")]
pub struct Trapframe {
    // registers as pushed by pusha
    pub edi: Uint,
    pub esi: Uint,
    pub ebp: Uint,
    pub oesp: Uint,      // useless & ignored
    pub ebx: Uint,
    pub edx: Uint,
    pub ecx: Uint,
    pub eax: Uint,
    // rest of trap frame
    pub gs: Ushort,
    pub padding1: Ushort,
    pub fs: Ushort,
    pub padding2: Ushort,
    pub es: Ushort,
    pub padding3: Ushort,
    pub ds: Ushort,
    pub padding4: Ushort,
    pub trapno: Uint,
    // below here defined by x86 hardware
    pub err: Uint,
    pub eip: Uint,
    pub cs: Ushort,
    pub padding5: Ushort,
    pub eflags: Uint,
    // below here only when crossing rings, such as from user to kernel
    pub esp: Uint,
    pub ss: Ushort,
    pub padding6: Ushort,
}