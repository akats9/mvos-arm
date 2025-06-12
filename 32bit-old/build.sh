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
arm-none-eabi-as -mcpu=arm1176jzf-s -g test-boot.s -o boot.o

echo "Building Rust kernel..."
cargo build --target armv7a-none-eabi --release

echo "Linking kernel..."
arm-none-eabi-ld -T test-linker.ld boot.o target/armv7a-none-eabi/release/libmvos_arm.a -o kernel.elf

echo "Creating binary..."
arm-none-eabi-objcopy -O binary kernel.elf kernel.bin

echo "Build complete! kernel.bin is ready."
read -r -p "Run kernel? [y/N] " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]
then
    qemu-system-arm \
    -M virt \
    -cpu cortex-a15 \
    -nographic \
    -kernel kernel.bin

else
    echo Exiting.
fi
