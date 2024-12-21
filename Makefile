# Compiler and Linker
CC = clang
AS = nasm
LD = ld

# CMAKE and compile_commands.json
CMAKE_COMMAND = cmake
COMPILE_COMMANDS = compile_commands.json

# Compiler Flags
CFLAGS = -ffreestanding \
         -O2 \
         -Wall \
         -Wextra \
         -Werror \
         -Wformat=2 \
         -fstack-protector-strong \
         -std=c23 \
         -m64 \
         -mno-red-zone
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
ASM_SRCS = $(shell find $(BOOT_DIR) $(KERNEL_DIR) -name '*.S')
C_SRCS = $(shell find $(KERNEL_DIR) -name '*.c')
HEADERS = $(shell find $(INCLUDE_DIR) -name '*.h')

# Generate object file names in build directory with unique suffixes
ASM_OBJS = $(ASM_SRCS:$(SRC_DIR)/%.S=$(BUILD_DIR)/%_asm.o)
C_OBJS = $(C_SRCS:$(SRC_DIR)/%.c=$(BUILD_DIR)/%_c.o)
OBJS = $(ASM_OBJS) $(C_OBJS)

# Create necessary build directories
$(shell mkdir -p $(BUILD_DIR)/boot $(BUILD_DIR)/kernel)

$(COMPILE_COMMANDS): $(OBJS)
	@echo "[" > $(COMPILE_COMMANDS)
	@cat $(BUILD_DIR)/kernel/*.o.json $(BUILD_DIR)/boot/*.o.json | sed '$$!s/$$/,/' >> $(COMPILE_COMMANDS)
	@echo "]" >> $(COMPILE_COMMANDS)

# Targets
all: $(KERNEL_BIN) $(COMPILE_COMMANDS)

$(KERNEL_BIN): $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $^

# Pattern rule for assembly files
$(BUILD_DIR)/%_asm.o: $(SRC_DIR)/%.S
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) $< -o $@

# Pattern rule for C files
$(BUILD_DIR)/%_c.o: $(SRC_DIR)/%.c $(HEADERS)
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
	@echo "C sources: $(C_SRCS)"
	@echo "Headers: $(HEADERS)"
	@echo "Objects: $(OBJS)"

.PHONY: all clean iso qemu debug
