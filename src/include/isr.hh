#pragma once
#include <stdint.h>

// This struct matches how the stack looks when an interrupt occurs
struct InterruptFrame {
    uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
    uint64_t rdi, rsi, rbp, rbx, rdx, rcx, rax;
    uint64_t interrupt_number, error_code;
    uint64_t rip, cs, rflags, rsp, ss;
} __attribute__((packed));

namespace ISR {
    // Function to initialize ISRs
    void initialize();

    // Handler that gets called from assembly
    extern "C" void handleException(InterruptFrame* frame);

    // Type for ISR handlers
    using HandlerFn = void (*)(InterruptFrame*);

    // Register a custom handler for an interrupt
    void registerHandler(uint8_t n, HandlerFn handler);
}
