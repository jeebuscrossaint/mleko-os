// src/spinlock.rs

// Dummy definitions for types used in the struct
#[repr(C)]
pub struct Cpu;

pub const PCS_SIZE: usize = 10;

// Mutual exclusion lock.
#[repr(C)]
pub struct Spinlock {
    pub locked: u32,             // Is the lock held?
    
    // For debugging:
    pub name: *const u8,         // Name of lock.
    pub cpu: *mut Cpu,           // The cpu holding the lock.
    pub pcs: [usize; PCS_SIZE],  // The call stack (an array of program counters)
                                 // that locked the lock.
}