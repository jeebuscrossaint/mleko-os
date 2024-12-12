#include "isr.hh"
#include "idt.hh"
#include "pic.hh"
#include "print.hh"

// Import the ISR stub table from assembly
extern "C" uint64_t isr_stub_table[];

// Array of custom handlers
static ISR::HandlerFn handlers[256] = {0};

void ISR::initialize() {
    Print::print("Initializing ISRs...\n");
    // Register first 32 ISRs (CPU exceptions)
    for (uint8_t i = 0; i < 32; i++) {
        IDT::setGate(i, isr_stub_table[i]);
        if (i == 0) {
            Print::print("Divide by zero handler installed\n");
        }
    }
    Print::print("ISRs initialized\n");
}

void ISR::registerHandler(uint8_t n, HandlerFn handler) {
    handlers[n] = handler;
}

void ISR::installIRQHandler(uint8_t irq, HandlerFn handler) {
    registerHandler(ISR::IRQ0 + irq, handler);
    PIC::enableIRQ(irq);
}

// This gets called from our ASM interrupt handler stub
extern "C" void handleException(InterruptFrame* frame) {
    // Handle IRQs (interrupt number >= 32 and < 48)
    if (frame->interrupt_number >= ISR::IRQ0 && frame->interrupt_number < ISR::IRQ0 + 16) {
        // Handle IRQ
        if (handlers[frame->interrupt_number]) {
            handlers[frame->interrupt_number](frame);
        }

        // Send EOI
        PIC::sendEOI(frame->interrupt_number - ISR::IRQ0);
        return;
    }

    // If we have a custom handler for other exceptions, use it
    if (handlers[frame->interrupt_number]) {
        handlers[frame->interrupt_number](frame);
        return;
    }

    // Default exception handling
    Print::print("\n=== EXCEPTION ===\n");
    Print::print("Exception Number: ");

    char num_str[3] = {0};
    int num = frame->interrupt_number;
    if (num < 10) {
        num_str[0] = '0' + num;
    } else {
        num_str[0] = '0' + (num / 10);
        num_str[1] = '0' + (num % 10);
    }
    Print::print(num_str);

    Print::print("\nRIP: 0x");
    char hex[16];  // Fixed size to 16
    uint64_t rip = frame->rip;
    for(int i = 15; i >= 0; i--) {
        uint8_t nibble = rip & 0xF;
        hex[i] = nibble + (nibble > 9 ? 'A' - 10 : '0');
        rip >>= 4;
    }
    hex[15] = '\0';
    Print::print(hex);

    Print::print("\nSystem Halted!\n");
    while(true) {
        asm volatile("hlt");
    }
}
