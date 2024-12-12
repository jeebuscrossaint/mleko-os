; kernel/idt.S
section .text
global idt_flush    ; Make the label accessible from C++

idt_flush:
    lidt [rdi]      ; Load IDT (first argument in rdi in x64)
    ret
