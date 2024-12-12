#include "idt.hh"
#include "print.hh"

IDTEntry IDT::entries[256];
IDTPtr IDT::ptr;

extern "C" void idt_flush(uint64_t);

void IDT::initialize() {
    // Set up the IDT pointer
    ptr.limit = (sizeof(IDTEntry) * 256) - 1;
    ptr.base = (uint64_t)&entries;

    // Clear out the entire IDT entries first
    for(uint16_t i = 0; i < 256; i++) {
        setGate(i, 0);
    }

    // Load the IDT
    idt_flush((uint64_t)&ptr);
}

void IDT::setGate(uint8_t num, uint64_t handler) {
    entries[num].offset_low = handler & 0xFFFF;
    entries[num].segment_selector = 0x08; // Kernel code segment
    entries[num].ist = 0;
    entries[num].type_attr = 0x8E; // Present, Ring0, Interrupt Gate
    entries[num].offset_mid = (handler >> 16) & 0xFFFF;
    entries[num].offset_high = (handler >> 32) & 0xFFFFFFFF;
    entries[num].zero = 0;

    if (num == 0) {
        Print::print("Setting up IDT entry 0 (Divide by Zero)\n");
    }
}
