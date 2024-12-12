#include "pic.hh"
#include <cstdint>
#include "print.hh"

#define PIC1_COMMAND 0x20
#define PIC1_DATA    0x21
#define PIC2_COMMAND 0xA0
#define PIC2_DATA    0xA1

// Simplified I/O operations
static inline void outb(uint16_t port, uint8_t val) {
    asm volatile ("out %0, %1" : : "a"(val), "d"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t ret;
    asm volatile ("in %1, %0" : "=a"(ret) : "d"(port));
    return ret;
}

namespace PIC {
    void initialize() {
        Print::print("Initializing PIC...\n");

        // ICW1: start initialization sequence
        outb(PIC1_COMMAND, 0x11);
        outb(PIC2_COMMAND, 0x11);

        // ICW2: set vector offset
        outb(PIC1_DATA, 0x20);  // IRQ 0-7: 32-39
        outb(PIC2_DATA, 0x28);  // IRQ 8-15: 40-47

        // ICW3: tell PICs about each other
        outb(PIC1_DATA, 4);
        outb(PIC2_DATA, 2);

        // ICW4: set mode
        outb(PIC1_DATA, 0x01);
        outb(PIC2_DATA, 0x01);

        Print::print("PIC initialized\n");
    }

    void disable() {
        // Mask all interrupts
        outb(PIC1_DATA, 0xFF);
        outb(PIC2_DATA, 0xFF);
    }

    void sendEOI(uint8_t irq) {
        if(irq >= 8)
            outb(PIC2_COMMAND, 0x20);
        outb(PIC1_COMMAND, 0x20);
    }
}
