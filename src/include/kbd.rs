// src/kbd.rs

pub const KBSTATP: u16 = 0x64;    // kbd controller status port(I)
pub const KBS_DIB: u8 = 0x01;     // kbd data in buffer
pub const KBDATAP: u16 = 0x60;    // kbd data port(I)

pub const NO: u8 = 0;

pub const SHIFT: u8 = 1 << 0;
pub const CTL: u8 = 1 << 1;
pub const ALT: u8 = 1 << 2;

pub const CAPSLOCK: u8 = 1 << 3;
pub const NUMLOCK: u8 = 1 << 4;
pub const SCROLLLOCK: u8 = 1 << 5;

pub const E0ESC: u8 = 1 << 6;

// Special keycodes
pub const KEY_HOME: u8 = 0xE0;
pub const KEY_END: u8 = 0xE1;
pub const KEY_UP: u8 = 0xE2;
pub const KEY_DN: u8 = 0xE3;
pub const KEY_LF: u8 = 0xE4;
pub const KEY_RT: u8 = 0xE5;
pub const KEY_PGUP: u8 = 0xE6;
pub const KEY_PGDN: u8 = 0xE7;
pub const KEY_INS: u8 = 0xE8;
pub const KEY_DEL: u8 = 0xE9;

// C('A') == Control-A
pub fn c(x: char) -> u8 {
    x as u8 - '@' as u8
}

pub static SHIFTCODE: [u8; 256] = {
    let mut arr = [NO; 256];
    arr[0x1D] = CTL;
    arr[0x2A] = SHIFT;
    arr[0x36] = SHIFT;
    arr[0x38] = ALT;
    arr[0x9D] = CTL;
    arr[0xB8] = ALT;
    arr
};

pub static TOGGLECODE: [u8; 256] = {
    let mut arr = [NO; 256];
    arr[0x3A] = CAPSLOCK;
    arr[0x45] = NUMLOCK;
    arr[0x46] = SCROLLLOCK;
    arr
};

pub static NORMALMAP: [u8; 256] = {
    let mut arr = [NO; 256];
    arr[0x00] = NO; arr[0x01] = 0x1B; arr[0x02] = '1' as u8; arr[0x03] = '2' as u8;
    arr[0x04] = '3' as u8; arr[0x05] = '4' as u8; arr[0x06] = '5' as u8; arr[0x07] = '6' as u8;
    arr[0x08] = '7' as u8; arr[0x09] = '8' as u8; arr[0x0A] = '9' as u8; arr[0x0B] = '0' as u8;
    arr[0x0C] = '-' as u8; arr[0x0D] = '=' as u8; arr[0x0E] = '\x08' as u8; arr[0x0F] = '\t' as u8;
    arr[0x10] = 'q' as u8; arr[0x11] = 'w' as u8; arr[0x12] = 'e' as u8; arr[0x13] = 'r' as u8;
    arr[0x14] = 't' as u8; arr[0x15] = 'y' as u8; arr[0x16] = 'u' as u8; arr[0x17] = 'i' as u8;
    arr[0x18] = 'o' as u8; arr[0x19] = 'p' as u8; arr[0x1A] = '[' as u8; arr[0x1B] = ']' as u8;
    arr[0x1C] = '\n' as u8; arr[0x1D] = NO; arr[0x1E] = 'a' as u8; arr[0x1F] = 's' as u8;
    arr[0x20] = 'd' as u8; arr[0x21] = 'f' as u8; arr[0x22] = 'g' as u8; arr[0x23] = 'h' as u8;
    arr[0x24] = 'j' as u8; arr[0x25] = 'k' as u8; arr[0x26] = 'l' as u8; arr[0x27] = ';' as u8;
    arr[0x28] = '\'' as u8; arr[0x29] = '`' as u8; arr[0x2A] = NO; arr[0x2B] = '\\' as u8;
    arr[0x2C] = 'z' as u8; arr[0x2D] = 'x' as u8; arr[0x2E] = 'c' as u8; arr[0x2F] = 'v' as u8;
    arr[0x30] = 'b' as u8; arr[0x31] = 'n' as u8; arr[0x32] = 'm' as u8; arr[0x33] = ',' as u8;
    arr[0x34] = '.' as u8; arr[0x35] = '/' as u8; arr[0x36] = NO; arr[0x37] = '*' as u8;
    arr[0x38] = NO; arr[0x39] = ' ' as u8; arr[0x3A] = NO; arr[0x3B] = NO;
    arr[0x3C] = NO; arr[0x3D] = NO; arr[0x3E] = NO; arr[0x3F] = NO;
    arr[0x40] = NO; arr[0x41] = NO; arr[0x42] = NO; arr[0x43] = NO;
    arr[0x44] = NO; arr[0x45] = NO; arr[0x46] = NO; arr[0x47] = '7' as u8;
    arr[0x48] = '8' as u8; arr[0x49] = '9' as u8; arr[0x4A] = '-' as u8; arr[0x4B] = '4' as u8;
    arr[0x4C] = '5' as u8; arr[0x4D] = '6' as u8; arr[0x4E] = '+' as u8; arr[0x4F] = '1' as u8;
    arr[0x50] = '2' as u8; arr[0x51] = '3' as u8; arr[0x52] = '0' as u8; arr[0x53] = '.' as u8;
    arr[0x9C] = '\n' as u8; arr[0xB5] = '/' as u8;
    arr[0xC8] = KEY_UP; arr[0xD0] = KEY_DN;
    arr[0xC9] = KEY_PGUP; arr[0xD1] = KEY_PGDN;
    arr[0xCB] = KEY_LF; arr[0xCD] = KEY_RT;
    arr[0x97] = KEY_HOME; arr[0xCF] = KEY_END;
    arr[0xD2] = KEY_INS; arr[0xD3] = KEY_DEL;
    arr
};

pub static SHIFTMAP: [u8; 256] = {
    let mut arr = [NO; 256];
    arr[0x00] = NO; arr[0x01] = 0x1B; arr[0x02] = '!' as u8; arr[0x03] = '@' as u8;
    arr[0x04] = '#' as u8; arr[0x05] = '$' as u8; arr[0x06] = '%' as u8; arr[0x07] = '^' as u8;
    arr[0x08] = '&' as u8; arr[0x09] = '*' as u8; arr[0x0A] = '(' as u8; arr[0x0B] = ')' as u8;
    arr[0x0C] = '_' as u8; arr[0x0D] = '+' as u8; arr[0x0E] = '\x08' as u8; arr[0x0F] = '\t' as u8;
    arr[0x10] = 'Q' as u8; arr[0x11] = 'W' as u8; arr[0x12] = 'E' as u8; arr[0x13] = 'R' as u8;
    arr[0x14] = 'T' as u8; arr[0x15] = 'Y' as u8; arr[0x16] = 'U' as u8; arr[0x17] = 'I' as u8;
    arr[0x18] = 'O' as u8; arr[0x19] = 'P' as u8; arr[0x1A] = '{' as u8; arr[0x1B] = '}' as u8;
    arr[0x1C] = '\n' as u8; arr[0x1D] = NO; arr[0x1E] = 'A' as u8; arr[0x1F] = 'S' as u8;
    arr[0x20] = 'D' as u8; arr[0x21] = 'F' as u8; arr[0x22] = 'G' as u8; arr[0x23] = 'H' as u8;
    arr[0x24] = 'J' as u8; arr[0x25] = 'K' as u8; arr[0x26] = 'L' as u8; arr[0x27] = ':' as u8;
    arr[0x28] = '"' as u8; arr[0x29] = '~' as u8; arr[0x2A] = NO; arr[0x2B] = '|' as u8;
    arr[0x2C] = 'Z' as u8; arr[0x2D] = 'X' as u8; arr[0x2E] = 'C' as u8; arr[0x2F] = 'V' as u8;
    arr[0x30] = 'B' as u8; arr[0x31] = 'N' as u8; arr[0x32] = 'M' as u8; arr[0x33] = '<' as u8;
    arr[0x34] = '>' as u8; arr[0x35] = '?' as u8; arr[0x36] = NO; arr[0x37] = '*' as u8;
    arr[0x38] = NO; arr[0x39] = ' ' as u8; arr[0x3A] = NO; arr[0x3B] = NO;
    arr[0x3C] = NO; arr[0x3D] = NO; arr[0x3E] = NO; arr[0x3F] = NO;
    arr[0x40] = NO; arr[0x41] = NO; arr[0x42] = NO; arr[0x43] = NO;
    arr[0x44] = NO; arr[0x45] = NO; arr[0x46] = NO; arr[0x47] = '7' as u8;
    arr[0x48] = '8' as u8; arr[0x49] = '9' as u8; arr[0x4A] = '-' as u8; arr[0x4B] = '4' as u8;
    arr[0x4C] = '5' as u8; arr[0x4D] = '6' as u8; arr[0x4E] = '+' as u8; arr[0x4F] = '1' as u8;
    arr[0x50] = '2' as u8; arr[0x51] = '3' as u8; arr[0x52] = '0' as u8; arr[0x53] = '.' as u8;
    arr[0x9C] = '\n' as u8; arr[0xB5] = '/' as u8;
    arr[0xC8] = KEY_UP; arr[0xD0] = KEY_DN;
    arr[0xC9] = KEY_PGUP; arr[0xD1] = KEY_PGDN;
    arr[0xCB] = KEY_LF; arr[0xCD] = KEY_RT;
    arr[0x97] = KEY_HOME; arr[0xCF] = KEY_END;
    arr[0xD2] = KEY_INS; arr[0xD3] = KEY_DEL;
    arr
};

pub static CTLMAP: [u8; 256] = {
    let mut arr = [NO; 256];
    arr[0x00] = NO; arr[0x01] = NO; arr[0x02] = NO; arr[0x03] = NO;
    arr[0x04] = NO; arr[0x05] = NO; arr[0x06] = NO; arr[0x07] = NO;
    arr[0x08] = NO; arr[0x09] = NO; arr[0x0A] = NO; arr[0x0B] = NO;
    arr[0x0C] = NO; arr[0x0D] = NO; arr[0x0E] = NO; arr[0x0F] = NO;
    arr[0x10] = c('Q'); arr[0x11] = c('W'); arr[0x12] = c('E'); arr[0x13] = c('R');
    arr[0x14] = c('T'); arr[0x15] = c('Y'); arr[0x16] = c('U'); arr[0x17] = c('I');
    arr[0x18] = c('O'); arr[0x19] = c('P'); arr[0x1A] = NO; arr[0x1B] = NO;
    arr[0x1C] = '\r' as u8; arr[0x1D] = NO; arr[0x1E] = c('A'); arr[0x1F] = c('S');
    arr[0x20] = c('D'); arr[0x21] = c('F'); arr[0x22] = c('G'); arr[0x23] = c('H');
    arr[0x24] = c('J'); arr[0x25] = c('K'); arr[0x26] = c('L'); arr[0x27] = NO;
    arr[0x28] = NO; arr[0x29] = NO; arr[0x2A] = NO; arr[0x2B] = c('\\');
    arr[0x2C] = c('Z'); arr[0x2D] = c('X'); arr[0x2E] = c('C'); arr[0x2F] = c('V');
    arr[0x30] = c('B'); arr[0x31] = c('N'); arr[0x32] = c('M'); arr[0x33] = NO;
    arr[0x34] = NO; arr[0x35] = c('/'); arr[0x36] = NO; arr[0x37] = NO;
    arr[0x9C] = '\r' as u8; arr[0xB5] = c('/');
    arr[0xC8] = KEY_UP; arr[0xD0] = KEY_DN;
    arr[0xC9] = KEY_PGUP; arr[0xD1] = KEY_PGDN;
    arr[0xCB] = KEY_LF; arr[0xCD] = KEY_RT;
    arr[0x97] = KEY_HOME; arr[0xCF] = KEY_END;
    arr[0xD2] = KEY_INS; arr[0xD3] = KEY_DEL;
    arr
};