#include "kernel.hh"
#include "print.hh"
#include "gdt.hh"
#include "idt.hh"
#include "isr.hh"

extern "C" void kernel_main() {
        GDT::initialize();
        IDT::initialize();
        ISR::initialize();

        // enable interrupts
        asm volatile("sti");

        Print::print("About to divide by zero");

        // test interrupt division by 0 error
        asm volatile(
                "mov $0, %%rcx\n"
                "div %%rcx" : : "a"(1) : "rcx"
            );

        Print::print("This should not be printed!\n");
        Print::clear();
        Print::print("Hello, ", Print::Color::LightBlue, Print::Color::Black);
        Print::print("World!\n", Print::Color::LightGreen, Print::Color::Black);
        Print::print("Welcome to Freax OS!", Print::Color::Yellow, Print::Color::Blue);

        while(1) {}
}
