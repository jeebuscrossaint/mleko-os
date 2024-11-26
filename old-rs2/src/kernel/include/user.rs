// src/user.rs

// Dummy definitions for types used in the function signatures
#[repr(C)]
pub struct Stat;

#[repr(C)]
pub struct Rtcdate;

// System calls
extern "C" {
    pub fn fork() -> i32;
    pub fn exit() -> !;
    pub fn wait() -> i32; // POSIX incompatible
    pub fn pipe(fds: *mut i32) -> i32;
    pub fn write(fd: i32, buf: *const u8, count: i32) -> i32;
    pub fn read(fd: i32, buf: *mut u8, count: i32) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn kill(pid: i32) -> i32; // POSIX incompatible
    pub fn exec(path: *const u8, argv: *const *const u8) -> i32; // POSIX incompatible
    pub fn open(path: *const u8, flags: i32) -> i32;
    pub fn mknod(path: *const u8, major: i16, minor: i16) -> i32;
    pub fn unlink(path: *const u8) -> i32;
    pub fn fstat(fd: i32, stat: *mut Stat) -> i32;
    pub fn link(oldpath: *const u8, newpath: *const u8) -> i32;
    pub fn mkdir(path: *const u8) -> i32; // POSIX incompatible
    pub fn chdir(path: *const u8) -> i32;
    pub fn dup(fd: i32) -> i32;
    pub fn getpid() -> i32;
    pub fn sbrk(incr: i32) -> *mut u8;
    pub fn sleep(seconds: i32) -> i32; // POSIX incompatible
    pub fn uptime() -> i32; // POSIX incompatible
    pub fn chmod(path: *const u8, mode: i32) -> i32;
}

// ulib.rs
extern "C" {
    pub fn stat(path: *const u8, stat: *mut Stat) -> i32;
    pub fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8;
    pub fn memmove(dest: *mut u8, src: *const u8, n: i32) -> *mut u8;
    pub fn strchr(s: *const u8, c: u8) -> *mut u8;
    pub fn strcmp(s1: *const u8, s2: *const u8) -> i32;
    pub fn printf(fd: i32, fmt: *const u8, ...);
    pub fn gets(buf: *mut u8, max: i32) -> *mut u8;
    pub fn strlen(s: *const u8) -> u32;
    pub fn memset(s: *mut u8, c: i32, n: u32) -> *mut u8;
    pub fn malloc(size: u32) -> *mut u8;
    pub fn free(ptr: *mut u8);
    pub fn atoi(s: *const u8) -> i32;
}