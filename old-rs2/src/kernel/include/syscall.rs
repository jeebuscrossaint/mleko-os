// src/syscall.rs

// System call numbers
pub const SYS_FORK: i32 = 1;
pub const SYS_EXIT: i32 = 2;
pub const SYS_WAIT: i32 = 3;
pub const SYS_PIPE: i32 = 4;
pub const SYS_READ: i32 = 5;
pub const SYS_KILL: i32 = 6;
pub const SYS_EXEC: i32 = 7;
pub const SYS_FSTAT: i32 = 8;
pub const SYS_CHDIR: i32 = 9;
pub const SYS_DUP: i32 = 10;
pub const SYS_GETPID: i32 = 11;
pub const SYS_SBRK: i32 = 12;
pub const SYS_SLEEP: i32 = 13;
pub const SYS_UPTIME: i32 = 14;
pub const SYS_OPEN: i32 = 15;
pub const SYS_WRITE: i32 = 16;
pub const SYS_MKNOD: i32 = 17;
pub const SYS_UNLINK: i32 = 18;
pub const SYS_LINK: i32 = 19;
pub const SYS_MKDIR: i32 = 20;
pub const SYS_CLOSE: i32 = 21;
pub const SYS_CHMOD: i32 = 22;