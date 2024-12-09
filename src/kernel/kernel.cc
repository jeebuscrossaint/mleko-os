#include "kernel.hh"

extern "C" void kernel_main() {
	volatile uint16_t* vram = (uint16_t*)0xB8000;

	const char* hello = "Hello, World!";
	for (int i = 0; hello[i] != '\0'; i++) {
		vram[i * 2] = hello[i] | 0x0F00; // white on black
	}

	while (true) {}
}
