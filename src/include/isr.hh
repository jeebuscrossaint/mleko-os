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

    constexpr uint8_t EXCEPTION_DE = 0;   // Divide Error
    constexpr uint8_t EXCEPTION_DB = 1;   // Debug
    constexpr uint8_t EXCEPTION_NMI = 2;  // Non-maskable Interrupt
    constexpr uint8_t EXCEPTION_BP = 3;   // Breakpoint

    // IRQ numbers (mapped to ISR 32-47)
    constexpr uint8_t IRQ0 = 32;  // Timer
    constexpr uint8_t IRQ1 = 33;  // Keyboard
    constexpr uint8_t IRQ2 = 34;  // Cascade

    void initialize();
    extern "C" void handleException(InterruptFrame* frame);
    using HandlerFn = void (*)(InterruptFrame*);
    void registerHandler(uint8_t n, HandlerFn handler);

    // New function to handle IRQs specifically
    void installIRQHandler(uint8_t irq, HandlerFn handler);
}
