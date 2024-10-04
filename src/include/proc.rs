// src/proc.rs

// Segments in proc->gdt.
pub const NSEGS: usize = 7;

// Per-CPU state
#[repr(C)]
pub struct Cpu {
    pub id: u8,                    // index into cpus[] below
    pub apicid: u8,                // Local APIC ID
    pub scheduler: *mut Context,   // swtch() here to enter scheduler
    pub ts: Taskstate,             // Used by x86 to find stack for interrupt
    pub gdt: [Segdesc; NSEGS],     // x86 global descriptor table
    pub started: u32,              // Has the CPU started?
    pub ncli: i32,                 // Depth of pushcli nesting.
    pub intena: i32,               // Were interrupts enabled before pushcli?
    #[cfg(target_pointer_width = "64")]
    pub local: *mut u8,            // Cpu-local storage variables
    #[cfg(target_pointer_width = "32")]
    pub cpu: *mut Cpu,             // Cpu-local storage variables
    #[cfg(target_pointer_width = "32")]
    pub proc: *mut Proc,           // The currently-running process.
}

extern "C" {
    pub static mut cpus: [Cpu; NCPU];
    pub static mut ncpu: i32;
}

#[cfg(target_pointer_width = "64")]
extern "C" {
    pub static mut cpu: *mut Cpu;
    pub static mut proc: *mut Proc;
}

#[cfg(target_pointer_width = "32")]
extern "C" {
    #[link_name = "%gs:0"]
    pub static mut cpu: *mut Cpu;       // &cpus[cpunum()]
    #[link_name = "%gs:4"]
    pub static mut proc: *mut Proc;     // cpus[cpunum()].proc
}

// Saved registers for kernel context switches.
#[cfg(target_pointer_width = "64")]
#[repr(C)]
pub struct Context {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub r11: usize,
    pub rbx: usize,
    pub ebp: usize, // rbp
    pub eip: usize, // rip
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
pub struct Context {
    pub edi: usize,
    pub esi: usize,
    pub ebx: usize,
    pub ebp: usize,
    pub eip: usize,
}

#[repr(C)]
pub enum Procstate {
    UNUSED,
    EMBRYO,
    SLEEPING,
    RUNNABLE,
    RUNNING,
    ZOMBIE,
}

// Per-process state
#[repr(C)]
pub struct Proc {
    pub sz: usize,                     // Size of process memory (bytes)
    pub pgdir: *mut pde_t,             // Page table
    pub kstack: *mut u8,               // Bottom of kernel stack for this process
    pub state: Procstate,              // Process state
    pub pid: i32,                      // Process ID
    pub parent: *mut Proc,             // Parent process
    pub tf: *mut Trapframe,            // Trap frame for current syscall
    pub context: *mut Context,         // swtch() here to run process
    pub chan: *mut u8,                 // If non-zero, sleeping on chan
    pub killed: i32,                   // If non-zero, have been killed
    pub ofile: [*mut File; NOFILE],    // Open files
    pub cwd: *mut Inode,               // Current directory
    pub name: [u8; 16],                // Process name (debugging)
}

// Dummy definitions for types used in the structs
#[repr(C)]
pub struct Taskstate;

#[repr(C)]
pub struct Segdesc;

#[repr(C)]
pub struct pde_t;

#[repr(C)]
pub struct Trapframe;

#[repr(C)]
pub struct File;

#[repr(C)]
pub struct Inode;

pub const NCPU: usize = 8;
pub const NOFILE: usize = 16;