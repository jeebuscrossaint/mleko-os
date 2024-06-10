
.set IRQ_BASE, 0x20

.section .text

.extern _ZN16InterruptManager15handleInterruptEhj



.macro handleException num
.global _ZN16InterruptManager16handleException\num\()Ev
    movb $\num, (interruptnumber)
    jmp int_bottom
.endm


.macro handleIntteruptRequest num
.global _ZN16InterruptManager26handleInterruptRequest\num\()Ev
    movb $\num + IRQ_BASE, (interruptnumber)
    jmp int_bottom
.endm


handleIntteruptRequest 0x00
handleIntteruptRequest 0x01

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

    iret

.data

    interruptnumber: .byte 0
