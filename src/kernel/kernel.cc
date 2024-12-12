#include "kernel.hh"
#include "print.hh"
#include "gdt.hh"
#include "idt.hh"
#include "isr.hh"
#include "pic.hh"

extern "C" void kernel_main() {
    GDT::initialize();
    IDT::initialize();
    ISR::initialize();

    // Remove the RFLAGS modification for now

    Print::clear();
    Print::print("Starting Freax OS...\n");

    PIC::initialize();
    PIC::disable();  // Make sure to disable for now

    Print::print("Hello, ", Print::Color::LightBlue, Print::Color::Black);
    Print::print("World!\n", Print::Color::LightGreen, Print::Color::Black);
    Print::print("Welcome to Freax OS!", Print::Color::Yellow, Print::Color::Blue);

    // Enable interrupts
     asm volatile("sti");

    while(1) {
        asm volatile("hlt");
    }
}
