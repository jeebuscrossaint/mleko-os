#pragma once
#include <stdint.h>

namespace PIC {
    // Standard IRQ numbers
    static const uint8_t IRQ_TIMER = 0;
    static const uint8_t IRQ_KEYBOARD = 1;
    static const uint8_t IRQ_CASCADE = 2;
    static const uint8_t IRQ_COM2 = 3;
    static const uint8_t IRQ_COM1 = 4;
    static const uint8_t IRQ_LPT2 = 5;
    static const uint8_t IRQ_FLOPPY = 6;
    static const uint8_t IRQ_LPT1 = 7;
    static const uint8_t IRQ_CMOS = 8;
    static const uint8_t IRQ_PS2MOUSE = 12;
    static const uint8_t IRQ_FPU = 13;
    static const uint8_t IRQ_ATA1 = 14;
    static const uint8_t IRQ_ATA2 = 15;

    void initialize();
    void disable();
    void sendEOI(uint8_t irq);

    // New functions
    void enableIRQ(uint8_t irq);
    void disableIRQ(uint8_t irq);
    bool isIRQEnabled(uint8_t irq);
}
