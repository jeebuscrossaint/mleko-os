#include <stdint.h>
#include <stddef.h>
#include "kernel.h"

// Function to write a character to VGA buffer
static void putchar(char c, uint8_t color, size_t x, size_t y) {
    volatile uint16_t* vga_buffer = (uint16_t*)0xB8000;
    size_t index = y * 80 + x;
    vga_buffer[index] = (uint16_t)c | ((uint16_t)color << 8);
}

// Simple function to write a string
static void print(const char* str, uint8_t color) {
    static size_t x = 0;
    static size_t y = 0;

    for (size_t i = 0; str[i] != '\0'; i++) {
        if (str[i] == '\n') {
            x = 0;
            y++;
            continue;
        }

        putchar(str[i], color, x, y);
        x++;

        if (x >= 80) {
            x = 0;
            y++;
        }

        if (y >= 25) {
            y = 0;
        }
    }
}

void kmain(void) {
    // Clear screen (fill with spaces)
    for (size_t y = 0; y < 25; y++) {
        for (size_t x = 0; x < 80; x++) {
            putchar(' ', 0x07, x, y);
        }
    }

    // Print welcome message
    print("Freax OS Booting...\n", 0x0F);
    print("Welcome to ", 0x0E);
    print("Freax", 0x0A);
    print("!\n", 0x0E);

    // Halt the CPU
    while(1) {
        __asm__ volatile("hlt");
    }
}
