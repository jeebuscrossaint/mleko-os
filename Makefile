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
BUILD_DIR = build

# Output
KERNEL_BIN = $(BUILD_DIR)/Freax.bin
ISO = $(BUILD_DIR)/Freax.iso

# Automatically find all source files
ASM_SRCS = $(shell find $(BOOT_DIR) -name '*.S')
CPP_SRCS = $(shell find $(KERNEL_DIR) -name '*.cc')
HEADERS = $(shell find $(INCLUDE_DIR) -name '*.hh')

# Generate object file names in build directory
ASM_OBJS = $(ASM_SRCS:$(SRC_DIR)/%.S=$(BUILD_DIR)/%.o)
CPP_OBJS = $(CPP_SRCS:$(SRC_DIR)/%.cc=$(BUILD_DIR)/%.o)
OBJS = $(ASM_OBJS) $(CPP_OBJS)

# Create necessary build directories
$(shell mkdir -p $(BUILD_DIR)/boot $(BUILD_DIR)/kernel)

# Targets
all: $(KERNEL_BIN)

$(KERNEL_BIN): $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

# Pattern rule for assembly files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.S
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

# Pattern rule for C++ files
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cc $(HEADERS)
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -I$(INCLUDE_DIR) -c $< -o $@

# ISO Creation
iso: $(KERNEL_BIN)
	mkdir -p $(BUILD_DIR)/isodir/boot/grub
	cp $(KERNEL_BIN) $(BUILD_DIR)/isodir/boot/
	echo "menuentry 'Freax' {" > $(BUILD_DIR)/isodir/boot/grub/grub.cfg
	echo "    multiboot2 /boot/Freax.bin" >> $(BUILD_DIR)/isodir/boot/grub/grub.cfg
	echo "}" >> $(BUILD_DIR)/isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) $(BUILD_DIR)/isodir

# QEMU Boot
qemu: iso
	qemu-system-x86_64 -cdrom $(ISO)

# Cleanup
clean:
	rm -rf $(BUILD_DIR)

# Debug printing
debug:
	@echo "Assembly sources: $(ASM_SRCS)"
	@echo "C++ sources: $(CPP_SRCS)"
	@echo "Headers: $(HEADERS)"
	@echo "Objects: $(OBJS)"

.PHONY: all clean iso qemu debug