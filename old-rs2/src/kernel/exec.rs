// src/kernel/exec.rs

use crate::include::types::*;
use crate::include::param::*;
use crate::include::memlayout::*;
use crate::include::mmu::*;
use crate::include::proc::*;
use crate::include::defs::*;
use crate::include::x86::*;
use crate::include::elf::*;

pub fn exec(path: &str, argv: &[&str]) -> i32 {
    let mut s: &str;
    let mut last: &str;
    let mut i: usize;
    let mut off: usize;
    let mut argc: usize;
    let mut sz: usize;
    let mut sp: usize;
    let mut ustack: [usize; 3 + MAXARG + 1] = [0; 3 + MAXARG + 1];
    let mut elf: Elfhdr = Elfhdr::default();
    let mut ip: Option<&mut Inode> = None;
    let mut ph: Proghdr = Proghdr::default();
    let mut pgdir: Option<&mut [Pde; NPDENTRIES]> = None;
    let mut oldpgdir: Option<&mut [Pde; NPDENTRIES]> = None;

    begin_op();
    if let Some(inode) = namei(path) {
        ip = Some(inode);
    } else {
        end_op();
        return -1;
    }
    ilock(ip.as_mut().unwrap());

    // Check ELF header
    if readi(ip.as_mut().unwrap(), &mut elf as *mut _ as *mut u8, 0, core::mem::size_of::<Elfhdr>()) < core::mem::size_of::<Elfhdr>() {
        goto_bad!(pgdir, ip);
    }
    if elf.magic != ELF_MAGIC {
        goto_bad!(pgdir, ip);
    }

    if let Some(pgdir_ptr) = setupkvm() {
        pgdir = Some(pgdir_ptr);
    } else {
        goto_bad!(pgdir, ip);
    }

    // Load program into memory.
    sz = 0;
    for i in 0..elf.phnum {
        off = elf.phoff + i * core::mem::size_of::<Proghdr>();
        if readi(ip.as_mut().unwrap(), &mut ph as *mut _ as *mut u8, off, core::mem::size_of::<Proghdr>()) != core::mem::size_of::<Proghdr>() {
            goto_bad!(pgdir, ip);
        }
        if ph.type_ != ELF_PROG_LOAD {
            continue;
        }
        if ph.memsz < ph.filesz {
            goto_bad!(pgdir, ip);
        }
        if allocuvm(pgdir.as_mut().unwrap(), sz, ph.vaddr + ph.memsz) == 0 {
            goto_bad!(pgdir, ip);
        }
        if loaduvm(pgdir.as_mut().unwrap(), ph.vaddr as *mut u8, ip.as_mut().unwrap(), ph.off, ph.filesz) < 0 {
            goto_bad!(pgdir, ip);
        }
    }
    iunlockput(ip.as_mut().unwrap());
    end_op();
    ip = None;

    // Allocate two pages at the next page boundary.
    // Make the first inaccessible. Use the second as the user stack.
    sz = PGROUNDUP!(sz);
    if allocuvm(pgdir.as_mut().unwrap(), sz, sz + 2 * PGSIZE) == 0 {
        goto_bad!(pgdir, ip);
    }
    clearpteu(pgdir.as_mut().unwrap(), (sz - 2 * PGSIZE) as *mut u8);
    sp = sz;

    // Push argument strings, prepare rest of stack in ustack.
    for argc in 0..argv.len() {
        if argc >= MAXARG {
            goto_bad!(pgdir, ip);
        }
        sp = (sp - (argv[argc].len() + 1)) & !(core::mem::size_of::<usize>() - 1);
        if copyout(pgdir.as_mut().unwrap(), sp, argv[argc].as_ptr(), argv[argc].len() + 1) < 0 {
            goto_bad!(pgdir, ip);
        }
        ustack[3 + argc] = sp;
    }
    ustack[3 + argc] = 0;

    ustack[0] = 0xffffffff; // fake return PC
    ustack[1] = argc;
    ustack[2] = sp - (argc + 1) * core::mem::size_of::<usize>();

    #[cfg(target_arch = "x86_64")]
    {
        proc().tf.rdi = argc;
        proc().tf.rsi = sp - (argc + 1) * core::mem::size_of::<usize>();
    }

    sp -= (3 + argc + 1) * core::mem::size_of::<usize>();
    if copyout(pgdir.as_mut().unwrap(), sp, ustack.as_ptr() as *const u8, (3 + argc + 1) * core::mem::size_of::<usize>()) < 0 {
        goto_bad!(pgdir, ip);
    }

    // Save program name for debugging.
    last = path;
    for s in path.chars() {
        if s == '/' {
            last = &path[s.len_utf8()..];
        }
    }
    safestrcpy(&mut proc().name, last, proc().name.len());

    // Commit to the user image.
    oldpgdir = Some(proc().pgdir);
    proc().pgdir = pgdir.unwrap();
    proc().sz = sz;
    proc().tf.eip = elf.entry; // main
    proc().tf.esp = sp;
    switchuvm(proc());
    freevm(oldpgdir.unwrap());
    return 0;

bad:
    if let Some(pgdir_ptr) = pgdir {
        freevm(pgdir_ptr);
    }
    if let Some(inode) = ip {
        iunlockput(inode);
        end_op();
    }
    -1
}