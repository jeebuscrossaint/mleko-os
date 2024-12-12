#pragma once
#include <stdint.h>

struct IDTPtr {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed));

struct IDTEntry {
    uint16_t offset_low;
    uint16_t segment_selector;
    uint8_t ist;        // only used in 64-bit mode
    uint8_t type_attr;
    uint16_t offset_mid;
    uint32_t offset_high;
    uint32_t zero;
} __attribute__((packed));

class IDT {
private:
    static IDTEntry entries[256];
    static IDTPtr ptr;

public:
    static void initialize();
    static void setGate(uint8_t num, uint64_t handler);  // Changed uint16_t to uint64_t
};
