.set IRQ_BASE, 0x20

.section .text

.extern _ZN16InterruptManager15handleInterruptEhj
.global _ZN16InterruptManager22ignoreExceptionEv


.macro handleException num
.global _ZN16InterruptManager16HandleException\num\()Ev
_ZN16InterruptManager16HandleException\num\()Ev:
    movb $\num, (interruptnumber)
    jmp int_bottom
.endm


.macro handleInterruptRequest num
.global _ZN16InterruptManager26HandleInterruptRequest\num\()Ev
_ZN16InterruptManager26HandleInterruptRequest\num\()Ev:
    movb $\num + IRQ_BASE, (interruptnumber)
    jmp int_bottom
.endm


handleInterruptRequest 0x00
handleInterruptRequest 0x01

# to add more interrupts you can copy these interrupt requests several more times so yea a lot of Assembly for sure!


int_bottom:

    pusha
    pushl %ds
    pushl %es
    pushl %fs
    pushl %gs

    pushl %esp
    push (interruptnumber)
    call _ZN16InterruptManager15handleInterruptEhj
    # addl $5, %esp
    movl %eax, %esp


    popl %gs
    popl %fs
    popl %es
    popl %ds
    popa
    pusha

_ZN16InterruptManager22ignoreExceptionEv:

    iret

.data

    interruptnumber: .byte 0
