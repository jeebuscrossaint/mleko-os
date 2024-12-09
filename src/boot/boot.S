global _start
extern kernel_main

section .multiboot
    ; Multiboot2 header
    align 8
    multiboot_header:
        dd 0xe85250d6         ; Magic number
        dd 0                  ; Architecture (0 = protected mode i386)
        dd multiboot_end - multiboot_header
        dd -(0xe85250d6 + 0 + (multiboot_end - multiboot_header))

    ; End tag
    dw 0    ; Type
    dw 0    ; Flags
    dd 8    ; Size
multiboot_end:

section .text
bits 32
_start:
    ; Disable interrupts
    cli

    ; Set up stack
    mov esp, stack_top

    ; Call kernel main
    call kernel_main

    ; Halt if kernel returns
    cli
    hlt

section .bss
    stack_bottom:
    resb 16384 ; 16 KB stack
    stack_top:
