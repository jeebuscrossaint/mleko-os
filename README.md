# Freax
*since Linux was originally going to be named Freax, I may as well do it myself.*

WE USE NASM SYNTAX FOR ASSEMBLY CODE. WRITTEN IN C++. CURRENTLY DONE WITH A BASIC MULTIBOOT BOOTLOADER. BASIC GDT IMPLEMENTED.

https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/

For a POSIX-compliant Unix-like OS, here's a suggested roadmap of key components to implement, roughly in order of priority:

1. **Memory Management**
   - Physical Memory Manager
   - Virtual Memory Manager (Paging)
   - Kernel Heap (kmalloc/kfree)
   - User Space Memory Management

2. **CPU Management & Protection**
   - GDT (Global Descriptor Table)
   - IDT (Interrupt Descriptor Table)
   - Basic exception handling
   - IRQ handling
   - User/Kernel mode separation

3. **Process Management**
   - Process structures
   - Task switching
   - Scheduler
   - Fork/Exec implementation
   - Signal handling (crucial for POSIX)

4. **File System**
   - VFS (Virtual File System) layer
   - Initial RAM disk
   - Basic file operations (open, read, write, close)
   - Implementation of a simple filesystem (ext2 is good to start with)
   - Device file system (/dev)

5. **System Calls**
   - Syscall interface
   - Basic POSIX syscalls
   - File operations
   - Process operations
   - Memory operations

6. **Device Drivers**
   - PIC (Programmable Interrupt Controller)
   - PIT (Programmable Interval Timer)
   - Keyboard driver
   - Storage drivers (IDE/SATA)
   - Basic networking stack

7. **IPC (Inter-Process Communication)**
   - Pipes
   - Signals
   - Shared memory
   - Message queues
   - Semaphores

8. **User Space**
   - C library implementation (or minimal libc)
   - Basic shell
   - Init system
   - Basic utilities (ls, cat, etc.)

For the immediate next step, I'd suggest starting with memory management since it's fundamental to everything else. Want me to help you set up a basic physical memory manager?

Some specific POSIX requirements to keep in mind:
- File system hierarchy standard (/bin, /etc, /dev, etc.)
- Standard I/O streams (stdin, stdout, stderr)
- Process groups and sessions
- File permissions and ownership
- Standard signal handling
- TTY/PTY support

This is a big project, but starting with memory management will give you a solid foundation to build upon.
