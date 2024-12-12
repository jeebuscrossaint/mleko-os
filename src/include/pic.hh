#pragma once
#include <stdint.h>

namespace PIC {
    void initialize();
    void disable();
    void sendEOI(uint8_t irq);
}
