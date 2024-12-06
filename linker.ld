ENTRY(_start)

SECTIONS
{
    /* Kernel starts at 0x80000 */
    . = 0x80000;

    .text : {
        KEEP(*(.text.boot))  /* Keep boot code at start */
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        . = ALIGN(16);
        __bss_start = .;
        *(.bss)
        *(COMMON)
        __bss_end = .;
    }

    /* Add a stack */
    . = ALIGN(16);
    . = . + 0x4000; /* 16KB stack */
    stack_top = .;

    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }
}
