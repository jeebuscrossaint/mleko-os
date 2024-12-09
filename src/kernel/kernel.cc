#include <cstdint>

extern "C" void kernel_main() {
    volatile uint16_t* video_memory = (uint16_t*)0xB8000;
    
    const char* hello = "Hello, World!";
    for(int i = 0; hello[i] != '\0'; i++) {
        video_memory[i * 2] = hello[i] | 0x0F00; // White text on black background
    }

    while(1) {} // Prevent CPU from executing random memory
}
