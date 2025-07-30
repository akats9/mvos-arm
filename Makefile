# Cross-compilation toolchain
CC = aarch64-elf-gcc
AR = aarch64-elf-ar
RANLIB = aarch64-elf-ranlib

# Directories
SRCDIR = src
BUILDDIR = build

# Compiler flags for C
CFLAGS = -Wall -Wextra -O2 -std=c99 -ffreestanding -nostdlib -nostdinc -I$(SRCDIR) -I$(INCLUDEDIR) -lm

# Find all .c files recursively in src directory
SOURCES = $(shell find $(SRCDIR) -name '*.c')

# Generate object files in build directory (maintaining directory structure)
# This strips src/ and adds build/ prefix
OBJECTS = $(patsubst $(SRCDIR)/%.c,$(BUILDDIR)/%.o,$(SOURCES))

# Target library
TARGET = $(BUILDDIR)/libckernel.a

# Default target
all: $(TARGET)

# Create the static library
$(TARGET): $(OBJECTS) | $(BUILDDIR)
	$(AR) rcs $@ $^
	$(RANLIB) $@

# Create all necessary directories upfront
OBJDIRS = $(sort $(dir $(OBJECTS)))

# Create directories before building
$(OBJECTS): | $(OBJDIRS)

$(OBJDIRS):
	@mkdir -p $@

# Compile source files to object files
$(BUILDDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

# Create build directory if it doesn't exist
$(BUILDDIR):
	mkdir -p $(BUILDDIR)

# Clean rule
clean:
	rm -rf $(BUILDDIR)

# Phony targets
.PHONY: all clean