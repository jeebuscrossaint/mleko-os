#include "gdt.hh"

// Define the static members with correct class scope
GDTEntry GDT::entries[5];  // Note GDT:: before both type and variable
GDTSystemEntry GDT::tss;   // Same here
GDTPtr GDT::ptr;          // And here

// Declare the assembly function
extern "C" void gdt_flush(uint64_t);

void GDT::initialize() {
    // Setup GDT pointer
    ptr.limit = (sizeof(GDTEntry) * 5 + sizeof(GDTSystemEntry)) - 1;
    ptr.base = (uint64_t)&entries;

    // Null descriptor
    entries[0] = {0, 0, 0, 0, 0, 0};

    // Kernel code segment
    entries[1] = {0, 0, 0, 0x9A, 0x20, 0}; // 0x9A = Present, Ring0, Code

    // Kernel data segment
    entries[2] = {0, 0, 0, 0x92, 0, 0};    // 0x92 = Present, Ring0, Data

    // User code segment
    entries[3] = {0, 0, 0, 0xFA, 0x20, 0}; // 0xFA = Present, Ring3, Code

    // User data segment
    entries[4] = {0, 0, 0, 0xF2, 0, 0};    // 0xF2 = Present, Ring3, Data

    // Load GDT
    gdt_flush((uint64_t)&ptr);
}
