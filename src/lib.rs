#![no_std]
#![no_main]
#![feature(slice_internals)]
#![feature(asm_const)]
#![feature(asm_sym)]

use core::{arch::asm, panic::PanicInfo};

extern crate alloc;

use crate::{memory::allocator::fixed_size_block::FixedSizeBlockAllocator, drivers::ramfb::setup_ramfb, exceptions::install_exception_handlers, framebuffer::{allocate_fb, fb_addr, gpu_init}, memory::mmu::mmu_init, uart::UartWriter};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_x0: u64, dtb_ptr: *const u8) -> ! {
    serial_println!("\x1B[1;32m[   INFO    ] Hello World!\x1B[0m");
    serial_println!("\x1B[1;32m[   INFO    ] MVOS aarch64 version 0.0.2\x1B[0m");

    serial_println!("[  KERNEL   ] Installing exception handlers...");
    unsafe { install_exception_handlers(); }

    serial_println!("[  KERNEL   ] Initializing MMU...");
    unsafe { mmu_init(); }

    serial_println!("[  KERNEL   ] Initializing allocator...");
    let mut allocator = FixedSizeBlockAllocator::new();
    unsafe { allocator.init(0x4000_0000, 20000); }

    serial_println!("[  DRIVERS  ] Initializing GPU...");

    unsafe { setup_ramfb(fb_addr as *mut u64, 800, 600); }

    serial_println!("[  DRIVERS  ] GPU Initialized.");

    unsafe { framebuffer::clear(0xFF); }

    serial_println!("[FRAMEBUFFER] Screen cleared");

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    //serial_print!("\x1B[1;31m[   PANIC   ] SYSTEM PANICKED\x1B[0m");
    unsafe {
        asm!("mov x0, #0x09000000");
        asm!("mov w1, #0x41");
        asm!("strb w1, [x0]"); 
    }
    loop{}
}

pub mod uart;
pub mod pci;
pub mod framebuffer;
pub mod drivers;
pub mod exceptions;
pub mod memory;