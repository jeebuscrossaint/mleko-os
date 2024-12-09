#include "kernel.hh"
#include "print.hh"

extern "C" void kernel_main() {
    Print::clear();
    Print::print("Hello, ", Print::Color::LightBlue, Print::Color::Black);
    Print::print("World!\n", Print::Color::LightGreen, Print::Color::Black);
    Print::print("Welcome to Freax OS!", Print::Color::Yellow, Print::Color::Blue);

    while(1) {}
}
