# Compiler and Linker
CC = g++
AS = nasm
LD = ld

# Compiler Flags
CFLAGS = -ffreestanding -O2 -Wall -Wextra -std=c++23 -m64 -fno-exceptions -fno-rtti
ASFLAGS = -f elf64
LDFLAGS = -nostdlib -T linker.ld

# Directories
SRC_DIR = src
BOOT_DIR = $(SRC_DIR)/boot
KERNEL_DIR = $(SRC_DIR)/kernel
INCLUDE_DIR = $(SRC_DIR)/include

# Output
KERNEL_BIN = Freax.bin
ISO = Freax.iso

# Source Files
BOOT_SRC = $(BOOT_DIR)/boot.S
KERNEL_SRC = $(KERNEL_DIR)/kernel.cc
KERNEL_HEADERS = $(INCLUDE_DIR)/kernel.hh

# Object Files
BOOT_OBJ = boot.o
KERNEL_OBJ = kernel.o

# Targets
all: $(KERNEL_BIN)

$(KERNEL_BIN): $(BOOT_OBJ) $(KERNEL_OBJ)
	$(LD) $(LDFLAGS) -o $@ $^

$(BOOT_OBJ): $(BOOT_SRC)
	$(AS) $(ASFLAGS) $< -o $@

$(KERNEL_OBJ): $(KERNEL_SRC) $(KERNEL_HEADERS)
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

# ISO Creation
iso:
	mkdir -p isodir/boot/grub
	cp Freax.bin isodir/boot/
	echo "menuentry 'Freax' {" > isodir/boot/grub/grub.cfg
	echo "    multiboot2 /boot/Freax.bin" >> isodir/boot/grub/grub.cfg
	echo "}" >> isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) isodir

# QEMU Boot
qemu: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO)

# Cleanup
clean:
	rm -f *.o $(KERNEL_BIN)
	rm -rf isodir $(ISO)

.PHONY: all clean iso qemu
