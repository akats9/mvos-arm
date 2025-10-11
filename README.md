# MVOS ARM
Arm operating system written in Rust (and C)

## Features
* Identity mapped paging and MMU support
* UART support for QEMU `virt` board
* RamFB GPU device support
* MVulkan GPU-agnostic graphics API (WIP)
* Visual console with color printing and support for UTF8 characters

## Tools 
Nightly tools used, these versions work (work used very loosely):
* rustc 1.91.0-nightly (9c27f27ea 2025-09-08)
* cargo 1.91.0-nightly (761c4658d 2025-09-04)
* aarch64-elf-gcc (GCC) 15.2.0
* (GNU Binutils) 2.45
* QEMU emulator version 10.1.0
* cbindgen 0.29.0