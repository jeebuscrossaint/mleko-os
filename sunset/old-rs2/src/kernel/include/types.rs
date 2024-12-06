// Type aliases
type uint = u32;
type ushort = u16;
type uchar = u8;

type uint32 = u32;
type uint64 = u64;

// Conditional type alias based on architecture
#[cfg(target_pointer_width = "64")]
type uintp = u64;

#[cfg(target_pointer_width = "32")]
type uintp = u32;

// Alias for page directory entry type
type pde_t = uintp;
