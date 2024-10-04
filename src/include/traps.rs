// x86 trap and interrupt constants.

// Processor-defined:
pub const T_DIVIDE: u32 = 0;      // divide error
pub const T_DEBUG: u32 = 1;       // debug exception
pub const T_NMI: u32 = 2;         // non-maskable interrupt
pub const T_BRKPT: u32 = 3;       // breakpoint
pub const T_OFLOW: u32 = 4;       // overflow
pub const T_BOUND: u32 = 5;       // bounds check
pub const T_ILLOP: u32 = 6;       // illegal opcode
pub const T_DEVICE: u32 = 7;      // device not available
pub const T_DBLFLT: u32 = 8;      // double fault
// pub const T_COPROC: u32 = 9;   // reserved (not used since 486)
pub const T_TSS: u32 = 10;        // invalid task switch segment
pub const T_SEGNP: u32 = 11;      // segment not present
pub const T_STACK: u32 = 12;      // stack exception
pub const T_GPFLT: u32 = 13;      // general protection fault
pub const T_PGFLT: u32 = 14;      // page fault
// pub const T_RES: u32 = 15;     // reserved
pub const T_FPERR: u32 = 16;      // floating point error
pub const T_ALIGN: u32 = 17;      // alignment check
pub const T_MCHK: u32 = 18;       // machine check
pub const T_SIMDERR: u32 = 19;    // SIMD floating point error

// These are arbitrarily chosen, but with care not to overlap
// processor defined exceptions or interrupt vectors.
pub const T_SYSCALL: u32 = 64;    // system call
pub const T_DEFAULT: u32 = 500;   // catchall

pub const T_IRQ0: u32 = 32;       // IRQ 0 corresponds to int T_IRQ

pub const IRQ_TIMER: u32 = 0;
pub const IRQ_KBD: u32 = 1;
pub const IRQ_COM1: u32 = 4;
pub const IRQ_IDE: u32 = 14;
pub const IRQ_ERROR: u32 = 19;
pub const IRQ_SPURIOUS: u32 = 31;
