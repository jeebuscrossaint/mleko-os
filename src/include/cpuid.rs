// src/include/cpuid.rs

pub const INTEL_PROC_FAMILY_SHIFT: u32 = 8;
pub const INTEL_PROC_FAMILY_MASK: u32 = 0xF << INTEL_PROC_FAMILY_SHIFT;
pub const INTEL_PROC_FAMILY_PENTIUM_4: u32 = 0xF;

pub const CPUID_LEAF_1_FPU: u32 = 1 << 0;
pub const CPUID_LEAF_1_VME: u32 = 1 << 1;
pub const CPUID_LEAF_1_DE: u32 = 1 << 2;
pub const CPUID_LEAF_1_PSE: u32 = 1 << 3;
pub const CPUID_LEAF_1_TSC: u32 = 1 << 4;
pub const CPUID_LEAF_1_MSR: u32 = 1 << 5;
pub const CPUID_LEAF_1_PAE: u32 = 1 << 6;
pub const CPUID_LEAF_1_MCE: u32 = 1 << 7;
pub const CPUID_LEAF_1_CX8: u32 = 1 << 8;
pub const CPUID_LEAF_1_APIC: u32 = 1 << 9;
pub const CPUID_LEAF_1_SEP: u32 = 1 << 11;
pub const CPUID_LEAF_1_MTRR: u32 = 1 << 12;
pub const CPUID_LEAF_1_PGE: u32 = 1 << 13;
pub const CPUID_LEAF_1_MCA: u32 = 1 << 14;
pub const CPUID_LEAF_1_CMOV: u32 = 1 << 15;
pub const CPUID_LEAF_1_PAT: u32 = 1 << 16;
pub const CPUID_LEAF_1_PSE36: u32 = 1 << 17;
pub const CPUID_LEAF_1_PSN: u32 = 1 << 18;
pub const CPUID_LEAF_1_CLFSH: u32 = 1 << 19;
pub const CPUID_LEAF_1_DS: u32 = 1 << 21;
pub const CPUID_LEAF_1_ACPI: u32 = 1 << 22;
pub const CPUID_LEAF_1_MMX: u32 = 1 << 23;
pub const CPUID_LEAF_1_FXSR: u32 = 1 << 24;
pub const CPUID_LEAF_1_SSE: u32 = 1 << 25;
pub const CPUID_LEAF_1_SSE2: u32 = 1 << 26;
pub const CPUID_LEAF_1_SS: u32 = 1 << 27;
pub const CPUID_LEAF_1_HTT: u32 = 1 << 28;
pub const CPUID_LEAF_1_TM: u32 = 1 << 29;
pub const CPUID_LEAF_1_PBE: u32 = 1 << 31;

pub const CPUID_LEAF_1_SSE3: u32 = 1 << 0;
pub const CPUID_LEAF_1_PCLMULQDQ: u32 = 1 << 1;
pub const CPUID_LEAF_1_DTES64: u32 = 1 << 2;
pub const CPUID_LEAF_1_MONITOR: u32 = 1 << 3;
pub const CPUID_LEAF_1_DS_CPL: u32 = 1 << 4;
pub const CPUID_LEAF_1_VMX: u32 = 1 << 5;
pub const CPUID_LEAF_1_SMX: u32 = 1 << 6;
pub const CPUID_LEAF_1_EIST: u32 = 1 << 7;
pub const CPUID_LEAF_1_TM2: u32 = 1 << 8;
pub const CPUID_LEAF_1_SSSE3: u32 = 1 << 9;
pub const CPUID_LEAF_1_CNXT_ID: u32 = 1 << 10;
pub const CPUID_LEAF_1_FMA: u32 = 1 << 12;
pub const CPUID_LEAF_1_CMPXCHG16B: u32 = 1 << 13;
pub const CPUID_LEAF_1_xTPR: u32 = 1 << 14;
pub const CPUID_LEAF_1_PDCM: u32 = 1 << 15;
pub const CPUID_LEAF_1_PCID: u32 = 1 << 17;
pub const CPUID_LEAF_1_DCA: u32 = 1 << 18;
pub const CPUID_LEAF_1_SSE4_1: u32 = 1 << 19;
pub const CPUID_LEAF_1_SSE4_2: u32 = 1 << 20;
pub const CPUID_LEAF_1_x2APIC: u32 = 1 << 21;
pub const CPUID_LEAF_1_MOVBE: u32 = 1 << 22;
pub const CPUID_LEAF_1_POPCNT: u32 = 1 << 23;
pub const CPUID_LEAF_1_TSCD: u32 = 1 << 24;
pub const CPUID_LEAF_1_AESNI: u32 = 1 << 25;
pub const CPUID_LEAF_1_XSAVE: u32 = 1 << 26;
pub const CPUID_LEAF_1_OSXSAVE: u32 = 1 << 27;
pub const CPUID_LEAF_1_AVX: u32 = 1 << 28;
pub const CPUID_LEAF_1_F16C: u32 = 1 << 29;
pub const CPUID_LEAF_1_RDRAND: u32 = 1 << 30;

pub const CPUID_LEAF_7_FSGSBASE: u32 = 1 << 0;
pub const CPUID_LEAF_7_TAM: u32 = 1 << 1;
pub const CPUID_LEAF_7_SMEP: u32 = 1 << 7;
pub const CPUID_LEAF_7_EREP: u32 = 1 << 9;
pub const CPUID_LEAF_7_INVPCID: u32 = 1 << 10;
pub const CPUID_LEAF_7_QM: u32 = 1 << 12;
pub const CPUID_LEAF_7_FPUCS: u32 = 1 << 13;

pub fn print_feature(flags: u32, feature: u32, feature_name: &str) {
    if flags & feature != 0 {
        print!("{} ", feature_name);
    }
}

pub fn get_stepping(a: u32) -> u32 {
    a & 0xF
}

pub fn get_model(a: u32) -> u32 {
    (a >> 4) & 0xF
}

pub fn get_family(a: u32) -> u32 {
    (a >> 8) & 0xF
}