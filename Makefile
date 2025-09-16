# ARM64 Kernel Build System - Single Unified Makefile
# Handles Rust, C, Assembly compilation and linking

# Configuration
TARGET := aarch64-unknown-none
KERNEL_NAME := kernel64
RUST_CRATE := mvos_arm

# Parameters
GPU ?= virtio-gpu-pci
MEMORY ?= 1G

# Tools
AS := aarch64-elf-as
LD := aarch64-elf-ld
OBJCOPY := aarch64-elf-objcopy
CC := aarch64-elf-gcc
AR := aarch64-elf-ar
CARGO := cargo

# Directories
BUILD_DIR := build
TARGET_DIR := target/$(TARGET)/release
SRC_DIR := src
INCLUDE_DIR := $(SRC_DIR)/include

# Files
BOOT_ASM := boot64.s
BOOT_OBJ := $(BUILD_DIR)/boot.o
LINKER_SCRIPT := linker64.ld
KERNEL_ELF := $(KERNEL_NAME).elf
KERNEL_BIN := $(KERNEL_NAME).bin
RUST_LIB := $(TARGET_DIR)/lib$(RUST_CRATE).a
C_LIB := $(BUILD_DIR)/libckernel.a
BINDINGS_HEADER := $(INCLUDE_DIR)/mvos_bindings.h

# Compilation flags
ASFLAGS := -g
CFLAGS := -ffreestanding -nostdlib -nostdinc \
          -mgeneral-regs-only -MMD -MP \
		  -g \
		  -O0 \
          -Wall -Wextra  \
          -I$(INCLUDE_DIR) \
          -mcpu=cortex-a72
LDFLAGS := -T $(LINKER_SCRIPT)

# Find sources (avoid any stray files)
RUST_SOURCES := $(shell find src -name "*.rs" 2>/dev/null || true)
C_SOURCES := $(shell find src -name "*.c" 2>/dev/null || true)
SX_SOURCES := $(shell find src -name "*.sx" 2>/dev/null || true)
C_OBJECTS := $(C_SOURCES:src/%.c=$(BUILD_DIR)/%.o)
SX_OBJECTS := $(SX_SOURCES:src/%.sx=$(BUILD_DIR)/%.o)
ALL_OBJECTS := $(C_OBJECTS) $(SX_OBJECTS)
C_DEPS := $(C_OBJECTS:.o=.d)

# Default target
.PHONY: all
all: $(KERNEL_ELF)

# Interactive build (like your original script)
.PHONY: interactive
interactive:
	@read -p "Clean before building? [y/N] " response; \
	if [[ "$$response" =~ ^([yY][eE][sS]|[yY])$$ ]]; then \
		$(MAKE) clean-all; \
		echo "Starting build - clean"; \
	else \
		echo "Starting build"; \
	fi
	@$(MAKE) all
	@read -p "Run kernel? [y/N] " response; \
	if [[ "$$response" =~ ^([yY][eE][sS]|[yY])$$ ]]; then \
		$(MAKE) run; \
	else \
		echo "Exiting."; \
	fi

# Create build directory
$(BUILD_DIR):
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(INCLUDE_DIR)
	@find src -type d 2>/dev/null | sed 's|src|$(BUILD_DIR)|' | xargs mkdir -p 2>/dev/null || true

# Step 1: Assembly compilation
$(BOOT_OBJ): $(BOOT_ASM) | $(BUILD_DIR)
	@echo "Assembling boot64.s..."
	$(AS) $(ASFLAGS) $< -o $@

# Step 2: Rust compilation
$(RUST_LIB): $(RUST_SOURCES) Cargo.toml
	@echo "Building Rust kernel..."
	$(CARGO) build --target $(TARGET) --release

# Step 3: Generate C bindings
$(BINDINGS_HEADER): $(RUST_LIB) cbindgen.toml | $(BUILD_DIR)
	@echo "Generating C bindings..."
	cbindgen --config cbindgen.toml --crate $(RUST_CRATE) --lang c --output $@

# Step 4: C object compilation
$(BUILD_DIR)/%.o: src/%.c $(BINDINGS_HEADER) | $(BUILD_DIR)
	@echo "Building C parts..."
	@echo "  CC    $<"
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# Step 4b: Assembly (.sx) compilation
$(BUILD_DIR)/%.o: src/%.sx | $(BUILD_DIR)
	@echo "  AS    $<"
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAFS) $< -o $@

# Step 5: Create C static library (if we have C sources)
$(C_LIB): $(ALL_OBJECTS) | $(BUILD_DIR)
	@if [ -n "$(ALL_OBJECTS)" ]; then \
		echo "Creating C library with $(words $(ALL_OBJECTS)) objects..."; \
		$(AR) rcs $@ $(ALL_OBJECTS); \
	else \
		echo "No C/.sx sources found, creating empty library..."; \
		$(AR) rcs $@; \
	fi

# Step 6: Final kernel linking
$(KERNEL_ELF): $(BOOT_OBJ) $(C_LIB) $(RUST_LIB) $(LINKER_SCRIPT)
	@echo "Linking kernel..."
	$(LD) $(LDFLAGS) $(BOOT_OBJ) \
		--whole-archive $(C_LIB) \
		--no-whole-archive \
		--whole-archive $(RUST_LIB) \
		--no-whole-archive \
		-o $@
	@echo "Build complete! $(KERNEL_ELF) is ready."

# Create binary image
$(KERNEL_BIN): $(KERNEL_ELF)
	@echo "Creating binary..."
	$(OBJCOPY) -O binary $< $@

# Phony targets
.PHONY: bin
bin: $(KERNEL_BIN)

.PHONY: run
run: $(KERNEL_ELF)
	@echo "Starting QEMU..."
	qemu-system-aarch64 \
		-M virt \
		-cpu cortex-a72 \
		-m $(MEMORY) \
		-kernel $< \
		-device $(GPU) \
		-serial stdio \
		-monitor unix:/tmp/qemu-monitor-socket,server,nowait 

.PHONY: debug
debug: $(KERNEL_ELF)
	@echo "Starting QEMU..."
	qemu-system-aarch64 \
		-M virt \
		-cpu cortex-a72 \
		-m $(MEMORY) \
		-kernel $< \
		-device $(GPU) \
		-serial stdio \
		-monitor unix:/tmp/qemu-monitor-socket,server,nowait \
		-s -S

# Clean targets
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	rm -f $(KERNEL_ELF) $(KERNEL_BIN)
	rm -rf $(BUILD_DIR)

.PHONY: clean-all
clean-all: clean
	@echo "Cleaning Cargo artifacts..."
	$(CARGO) clean
	rm -f $(BINDINGS_HEADER)

# Rebuild everything from scratch
.PHONY: rebuild
rebuild: clean-all all

# Generate bindings
.PHONY: bindings
bindings: $(BINDINGS_HEADER)

# Debug information
.PHONY: info
info:
	@echo "Build configuration:"
	@echo "  Target: $(TARGET)"
	@echo "  Kernel ELF: $(KERNEL_ELF)"
	@echo "  Rust lib: $(RUST_LIB)"
	@echo "  C lib: $(C_LIB)"
	@echo "  Boot object: $(BOOT_OBJ)"
	@echo "  Linker script: $(LINKER_SCRIPT)"
	@echo ""
	@echo "Sources found:"
	@echo "  Rust sources: $(words $(RUST_SOURCES)) files"
	@echo "  C sources: $(words $(C_SOURCES)) files"
	@if [ -n "$(C_SOURCES)" ]; then \
		echo "  C source files:"; \
		for src in $(C_SOURCES); do echo "    $$src"; done; \
	fi

# Check tools
.PHONY: check-tools
check-tools:
	@echo "Checking build tools..."
	@which $(AS) > /dev/null || (echo "ERROR: $(AS) not found" && exit 1)
	@which $(CC) > /dev/null || (echo "ERROR: $(CC) not found" && exit 1)
	@which $(LD) > /dev/null || (echo "ERROR: $(LD) not found" && exit 1)
	@which $(AR) > /dev/null || (echo "ERROR: $(AR) not found" && exit 1)
	@which $(CARGO) > /dev/null || (echo "ERROR: $(CARGO) not found" && exit 1)
	@which cbindgen > /dev/null || (echo "ERROR: cbindgen not found" && exit 1)
	@which qemu-system-aarch64 > /dev/null || (echo "WARNING: qemu-system-aarch64 not found")
	@echo "All required tools found!"

# Help
.PHONY: help
help:
	@echo "ARM64 Kernel Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  all            - Build kernel (default)"
	@echo "  interactive    - Interactive build (like your original script)"
	@echo "  bin            - Build kernel binary image"
	@echo "  run            - Run kernel in QEMU"
	@echo "  debug          - Run kernel in QEMU and await GDB"
	@echo "  clean          - Clean build artifacts"
	@echo "  clean-all      - Clean all artifacts including Cargo"
	@echo "  rebuild        - Clean and rebuild everything"
	@echo "  bindings       - Generate bindings header"
	@echo "  info           - Show build configuration"
	@echo "  check-tools    - Verify all required tools are available"
	@echo "  help           - Show this help"
	@echo ""
	@echo "Example usage:"
	@echo "  make interactive    # Mimics your original build script"
	@echo "  make all && make run    # Build and run"
	@echo "  make clean-all && make    # Clean rebuild"

# Include dependency files for C sources
-include $(C_DEPS)
