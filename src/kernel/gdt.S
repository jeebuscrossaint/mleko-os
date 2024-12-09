; gdt.S
section .text
global gdt_flush    ; Make the label accessible from C++

gdt_flush:
    lgdt [rdi]      ; Load GDT (first argument in rdi in x64)

    ; Update data segments
    mov ax, 0x10    ; Kernel data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ; Far return to update CS
    push qword 0x08 ; Kernel code segment
    push qword reload_cs ; Return address
    retfq           ; Far return
reload_cs:
    ret
