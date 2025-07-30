#!/bin/bash

# Build script for ARM kernel

set -e  # Exit on any error

read -r -p "Clean before building? [y/N] " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]
then
    cargo clean
    make clean
    echo "Starting build - clean"
else
    echo "Starting build"
fi

echo "Assembling boot.s..."
aarch64-elf-as -g boot64.s -o boot.o

echo "Building Rust kernel..."
cargo build --target aarch64-unknown-none --release

echo "Generating C bindings..."
cbindgen --config cbindgen.toml --crate mvos_arm --lang c --output src/include/mvos_bindings.h

echo "Building C parts..."
make 

echo "Linking kernel..."
aarch64-elf-ld -T linker64.ld boot.o --whole-archive build/libckernel.a --no-whole-archive target/aarch64-unknown-none/release/libmvos_arm.a -o kernel64.elf

# echo "Creating binary..."
# aarch64-elf-objcopy -O binary kernel.elf kernel.bin

echo "Build complete! kernel.elf is ready."
read -r -p "Run kernel? [y/N] " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]
then
    qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a72 \
    -m 1G \
    -kernel kernel64.elf \
    -device virtio-gpu-pci \
    -serial stdio \
    -monitor unix:/tmp/qemu-monitor-socket,server,nowait #-s -S

else
    echo Exiting.
fi
