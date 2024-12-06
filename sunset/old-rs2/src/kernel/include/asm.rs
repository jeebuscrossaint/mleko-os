#![feature(asm)]

/// Constants for segment types
pub const STA_X: u8 = 0x8; // Executable segment
pub const STA_E: u8 = 0x4; // Expand down (non-executable segments)
pub const STA_C: u8 = 0x4; // Conforming code segment (executable only)
pub const STA_W: u8 = 0x2; // Writeable (non-executable segments)
pub const STA_R: u8 = 0x2; // Readable (executable segments)
pub const STA_A: u8 = 0x1; // Accessed

/// Macro to create a null segment
#[macro_export]
macro_rules! seg_nullasm {
    () => {
        asm!(
            ".word 0, 0",
            ".byte 0, 0, 0, 0",
            options(att_syntax)
        );
    };
}

/// Macro to create a segment descriptor
#[macro_export]
macro_rules! seg_asm {
    ($type:expr, $base:expr, $lim:expr) => {
        asm!(
            ".word {lim_low}, {base_low}",
            ".byte {base_mid}, {type}, {lim_high}, {base_high}",
            lim_low = const (($lim >> 12) & 0xffff) as u16,
            base_low = const ($base & 0xffff) as u16,
            base_mid = const (($base >> 16) & 0xff) as u8,
            type = const (0x90 | $type) as u8,
            lim_high = const (0xC0 | (($lim >> 28) & 0xf)) as u8,
            base_high = const (($base >> 24) & 0xff) as u8,
            options(att_syntax)
        );
    };
}

