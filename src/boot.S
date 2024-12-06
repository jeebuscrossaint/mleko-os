.section ".text.boot"
.global _start

_start:
    // Only run on CPU 0
    mrs     x0, mpidr_el1
    and     x0, x0, #3
    cbz     x0, 2f
1:  wfe
    b       1b
2:
    // Set up stack
    ldr     x0, =stack_top
    mov     sp, x0

    // Clear BSS
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
    sub     x1, x1, x0
3:  cbz     x1, 4f
    str     xzr, [x0], #8
    sub     x1, x1, #8
    b       3b

4:  // Jump to kernel_main
    bl      kernel_main

    // If kernel_main returns, halt
5:  wfe
    b       5b
