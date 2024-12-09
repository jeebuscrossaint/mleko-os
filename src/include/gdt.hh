#pragma once
#include <stdint.h>

// Move structs outside the class
struct GDTEntry {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t base_middle;
    uint8_t access;
    uint8_t granularity;
    uint8_t base_high;
} __attribute__((packed));

struct GDTSystemEntry {
    uint16_t limit_low;
    uint16_t base_low;
    uint8_t base_middle;
    uint8_t access;
    uint8_t granularity;
    uint8_t base_high;
    uint32_t base_upper;
    uint32_t reserved;
} __attribute__((packed));

struct GDTPtr {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed));

class GDT {
private:
    static GDTEntry entries[5];
    static GDTSystemEntry tss;
    static GDTPtr ptr;

public:
    static void initialize();
};
