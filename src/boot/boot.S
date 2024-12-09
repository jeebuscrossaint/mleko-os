global _start
extern kernel_main

section .multiboot
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

    ; Set up page tables
    mov eax, page_table_l3
    or eax, 0b11    ; Present + writable
    mov [page_table_l4], eax

    mov eax, page_table_l2
    or eax, 0b11    ; Present + writable
    mov [page_table_l3], eax

    ; Map first GB with 2MB pages
    mov ecx, 0
.map_l2:
    mov eax, 0x200000    ; 2MB
    mul ecx
    or eax, 0b10000011   ; Present + writable + huge
    mov [page_table_l2 + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .map_l2

    ; Enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Set long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; Load page table
    mov eax, page_table_l4
    mov cr3, eax

    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; Load GDT
    lgdt [gdt64.pointer]
    jmp gdt64.code:long_mode_start

bits 64
long_mode_start:
    ; Set up segment registers
    mov ax, gdt64.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Set up stack
    mov rsp, stack_top

    ; Call kernel
    call kernel_main

    ; Halt if kernel returns
    cli
    hlt

section .bss
align 4096
page_table_l4:
    resb 4096
page_table_l3:
    resb 4096
page_table_l2:
    resb 4096

stack_bottom:
    resb 16384 ; 16 KB stack
stack_top:

section .rodata
gdt64:
    dq 0 ; Zero entry
.code: equ $ - gdt64
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; Code segment
.data: equ $ - gdt64
    dq (1 << 44) | (1 << 47) | (1 << 41) ; Data segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
