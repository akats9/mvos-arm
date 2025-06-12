#!/bin/bash

#!/bin/bash

# Build script for ARM kernel

set -e  # Exit on any error

read -r -p "Clean before building? [y/N] " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]
then
    cargo clean
    echo "Starting build - clean"
else
    echo "Starting build"
fi

echo "Assembling boot.s..."
aarch64-elf-as -g boot64.s -o boot.o

echo "Building Rust kernel..."
cargo build --target aarch64-unknown-none --release

echo "Linking kernel..."
aarch64-elf-ld -T test-linker.ld boot.o target/aarch64-unknown-none/release/libmvos_arm.a -o kernel64.elf

# echo "Creating binary..."
# aarch64-elf-objcopy -O binary kernel.elf kernel.bin

echo "Build complete! kernel.elf is ready."
read -r -p "Run kernel? [y/N] " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]
then
    qemu-system-aarch64 \
    -M virt \
    -cpu cortex-a57 \
    -nographic \
    -kernel kernel64.elf

else
    echo Exiting.
fi
