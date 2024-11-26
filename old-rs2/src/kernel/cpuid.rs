// src/cpuid.rs

use crate::include::types::*;
use crate::include::defs::*;
use crate::include::fs::*;
use crate::include::file::*;
use crate::include::cpuid::*;

static mut MAXLEAF: u32 = 0;
static mut VENDOR: [u32; 3] = [0; 3];
// leaf = 1
static mut VERSION: u32 = 0;
static mut PROCESSOR: u32 = 0;
static mut FEATURESEXT: u32 = 0;
static mut FEATURES: u32 = 0;
// leaf = 7
static mut SEF_FLAGS: u32 = 0;

fn cpu_printfeatures() {
    let mut vendor_str = [0u8; 13];
    unsafe {
        ptr::write_unaligned(&mut vendor_str[0] as *mut u8 as *mut u32, VENDOR[0]);
        ptr::write_unaligned(&mut vendor_str[4] as *mut u8 as *mut u32, VENDOR[1]);
        ptr::write_unaligned(&mut vendor_str[8] as *mut u8 as *mut u32, VENDOR[2]);
    }
    vendor_str[12] = 0;

    cprintf!("CPU vendor: {}\n", core::str::from_utf8(&vendor_str).unwrap());
    unsafe {
        cprintf!("Max leaf: 0x{:x}\n", MAXLEAF);
        if MAXLEAF >= 1 {
            cprintf!("Features: ");
            PRINT_FEATURE!(FEATURES, FPU);
            PRINT_FEATURE!(FEATURES, VME);
            PRINT_FEATURE!(FEATURES, DE);
            PRINT_FEATURE!(FEATURES, PSE);
            PRINT_FEATURE!(FEATURES, TSC);
            PRINT_FEATURE!(FEATURES, MSR);
            PRINT_FEATURE!(FEATURES, PAE);
            PRINT_FEATURE!(FEATURES, MCE);
            PRINT_FEATURE!(FEATURES, CX8);
            PRINT_FEATURE!(FEATURES, APIC);
            PRINT_FEATURE!(FEATURES, SEP);
            PRINT_FEATURE!(FEATURES, MTRR);
            PRINT_FEATURE!(FEATURES, PGE);
            PRINT_FEATURE!(FEATURES, MCA);
            PRINT_FEATURE!(FEATURES, CMOV);
            PRINT_FEATURE!(FEATURES, PAT);
            PRINT_FEATURE!(FEATURES, PSE36);
            PRINT_FEATURE!(FEATURES, PSN);
            PRINT_FEATURE!(FEATURES, CLFSH);
            PRINT_FEATURE!(FEATURES, DS);
            PRINT_FEATURE!(FEATURES, ACPI);
            PRINT_FEATURE!(FEATURES, MMX);
            PRINT_FEATURE!(FEATURES, FXSR);
            PRINT_FEATURE!(FEATURES, SSE);
            PRINT_FEATURE!(FEATURES, SSE2);
            PRINT_FEATURE!(FEATURES, SS);
            PRINT_FEATURE!(FEATURES, HTT);
            PRINT_FEATURE!(FEATURES, TM);
            PRINT_FEATURE!(FEATURES, PBE);

            cprintf!("\nExt Features: ");
            PRINT_FEATURE!(FEATURESEXT, SSE3);
            PRINT_FEATURE!(FEATURESEXT, PCLMULQDQ);
            PRINT_FEATURE!(FEATURESEXT, DTES64);
            PRINT_FEATURE!(FEATURESEXT, MONITOR);
            PRINT_FEATURE!(FEATURESEXT, DS_CPL);
            PRINT_FEATURE!(FEATURESEXT, VMX);
            PRINT_FEATURE!(FEATURESEXT, SMX);
            PRINT_FEATURE!(FEATURESEXT, EIST);
            PRINT_FEATURE!(FEATURESEXT, TM2);
            PRINT_FEATURE!(FEATURESEXT, SSSE3);
            PRINT_FEATURE!(FEATURESEXT, CNXT_ID);
            PRINT_FEATURE!(FEATURESEXT, FMA);
            PRINT_FEATURE!(FEATURESEXT, CMPXCHG16B);
            PRINT_FEATURE!(FEATURESEXT, xTPR);
            PRINT_FEATURE!(FEATURESEXT, PDCM);
            PRINT_FEATURE!(FEATURESEXT, PCID);
            PRINT_FEATURE!(FEATURESEXT, DCA);
            PRINT_FEATURE!(FEATURESEXT, SSE4_1);
            PRINT_FEATURE!(FEATURESEXT, SSE4_2);
            PRINT_FEATURE!(FEATURESEXT, x2APIC);
            PRINT_FEATURE!(FEATURESEXT, MOVBE);
            PRINT_FEATURE!(FEATURESEXT, POPCNT);
            PRINT_FEATURE!(FEATURESEXT, TSCD);
            PRINT_FEATURE!(FEATURESEXT, AESNI);
            PRINT_FEATURE!(FEATURESEXT, XSAVE);
            PRINT_FEATURE!(FEATURESEXT, OSXSAVE);
            PRINT_FEATURE!(FEATURESEXT, AVX);
            PRINT_FEATURE!(FEATURESEXT, F16C);
            PRINT_FEATURE!(FEATURESEXT, RDRAND);
            cprintf!("\n");
        }

        if MAXLEAF >= 7 {
            cprintf!("Structured Extended Features: ");
            PRINT_SEFEATURE!(SEF_FLAGS, FSGSBASE);
            PRINT_SEFEATURE!(SEF_FLAGS, TAM);
            PRINT_SEFEATURE!(SEF_FLAGS, SMEP);
            PRINT_SEFEATURE!(SEF_FLAGS, EREP);
            PRINT_SEFEATURE!(SEF_FLAGS, INVPCID);
            PRINT_SEFEATURE!(SEF_FLAGS, QM);
            PRINT_SEFEATURE!(SEF_FLAGS, FPUCS);
            cprintf!("\n");
        }
    }
}

fn cpuinfo() {
    unsafe {
        // check for CPUID support by setting and clearing ID (bit 21) in EFLAGS

        // When EAX=0, the processor returns the highest value (maxleaf) recognized for processor information
        asm!(
            "cpuid",
            out("eax") MAXLEAF,
            out("ebx") VENDOR[0],
            out("ecx") VENDOR[2],
            out("edx") VENDOR[1],
            in("eax") 0,
        );

        if MAXLEAF >= 1 {
            // get model, family, stepping info
            asm!(
                "cpuid",
                out("eax") VERSION,
                out("ebx") PROCESSOR,
                out("ecx") FEATURESEXT,
                out("edx") FEATURES,
                in("eax") 1,
            );
        }

        if MAXLEAF >= 2 {
            // cache and TLB info
        }

        if MAXLEAF >= 3 {
            // processor serial number
        }

        if MAXLEAF >= 4 {
            // deterministic cache parameters
        }

        if MAXLEAF >= 5 {
            // MONITOR and MWAIT instructions
        }

        if MAXLEAF >= 6 {
            // thermal and power management
        }

        if MAXLEAF >= 7 {
            // structured extended feature flags (ECX=0)
            let mut maxsubleaf: u32 = 0;
            asm!(
                "cpuid",
                out("eax") maxsubleaf,
                out("ebx") SEF_FLAGS,
                in("eax") 7,
                in("ecx") 0,
            );
        }

        /* ... and many more ... */
    }
}

fn cpuid_read(_i: &mut Inode, _buf: &mut [u8], _count: usize) -> i32 {
    cpu_printfeatures();
    0
}

fn cpuid_write(_i: &mut Inode, _buf: &[u8], _count: usize) -> i32 {
    cprintf!("cpuid_write\n");
    0
}

pub fn cpuidinit() {
    unsafe {
        devsw[CPUID].write = cpuid_write;
        devsw[CPUID].read = cpuid_read;

        cpuinfo();
    }
}