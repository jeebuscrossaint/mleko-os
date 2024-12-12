#include "isr.hh"
#include "idt.hh"
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
            Print::print("Divide by zero handler installed at: ");
            // You might want to add a function to print hex numbers here
        }
    }
    Print::print("ISRs initialized\n");
}

void ISR::registerHandler(uint8_t n, HandlerFn handler) {
    handlers[n] = handler;
}

// This gets called from our ASM interrupt handler stub
extern "C" void handleException(InterruptFrame* frame) {
    // If we have a custom handler, call it
    if (handlers[frame->interrupt_number]) {
        handlers[frame->interrupt_number](frame);
        return;
    }

    // Print debug info
    Print::print("Exception occurred! Number: ");

    // Convert interrupt number to string (basic implementation)
    char num_str[3] = {0};
    int num = frame->interrupt_number;
    if (num < 10) {
        num_str[0] = '0' + num;
    } else {
        num_str[0] = '0' + (num / 10);
        num_str[1] = '0' + (num % 10);
    }
    Print::print(num_str);
    Print::print("\nSystem Halted!\n");

    // Halt the system
    while(true) {
        asm volatile("hlt");
    }
}
