global isr_stub_table

extern handleException

; Common ISR stub that calls C++ handler
isr_common_stub:
    ; Push all general purpose registers
    push rax
    push rcx
    push rdx
    push rbx
    push rbp
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; First argument to function (rdi) is the interrupt frame
    mov rdi, rsp

    ; Call C++ handler
    call handleException

    ; Restore all registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rbp
    pop rbx
    pop rdx
    pop rcx
    pop rax

    ; Remove error code and interrupt number
    add rsp, 16
    iretq

; Macro for ISRs that don't push an error code
%macro ISR_NOERRCODE 1
isr%1:
    push qword 0     ; Push dummy error code
    push qword %1    ; Push interrupt number
    jmp isr_common_stub
%endmacro

; Macro for ISRs that do push an error code
%macro ISR_ERRCODE 1
isr%1:
    push qword %1    ; Push interrupt number
    jmp isr_common_stub
%endmacro

; Generate ISRs for all CPU exceptions
ISR_NOERRCODE 0  ; Divide by zero
ISR_NOERRCODE 1  ; Debug
ISR_NOERRCODE 2  ; Non-maskable interrupt
ISR_NOERRCODE 3  ; Breakpoint
ISR_NOERRCODE 4  ; Overflow
ISR_NOERRCODE 5  ; Bound range exceeded
ISR_NOERRCODE 6  ; Invalid opcode
ISR_NOERRCODE 7  ; Device not available
ISR_ERRCODE   8  ; Double fault
ISR_NOERRCODE 9  ; Coprocessor segment overrun
ISR_ERRCODE   10 ; Invalid TSS
ISR_ERRCODE   11 ; Segment not present
ISR_ERRCODE   12 ; Stack-segment fault
ISR_ERRCODE   13 ; General protection fault
ISR_ERRCODE   14 ; Page fault
ISR_NOERRCODE 15 ; Reserved
ISR_NOERRCODE 16 ; x87 FPU error
ISR_ERRCODE   17 ; Alignment check
ISR_NOERRCODE 18 ; Machine check
ISR_NOERRCODE 19 ; SIMD floating-point
ISR_NOERRCODE 20 ; Virtualization
ISR_NOERRCODE 21 ; Control protection
ISR_NOERRCODE 22 ; Reserved
ISR_NOERRCODE 23 ; Reserved
ISR_NOERRCODE 24 ; Reserved
ISR_NOERRCODE 25 ; Reserved
ISR_NOERRCODE 26 ; Reserved
ISR_NOERRCODE 27 ; Reserved
ISR_NOERRCODE 28 ; Reserved
ISR_NOERRCODE 29 ; Reserved
ISR_ERRCODE   30 ; Security exception
ISR_NOERRCODE 31 ; Reserved

; Create a table of ISR addresses
section .data
isr_stub_table:
%assign i 0
%rep 32
    dq isr%+i
%assign i i+1
%endrep
