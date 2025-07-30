#![no_std]
#![no_main]
#![feature(slice_internals)]
#![feature(asm_const)]
#![feature(asm_sym)]

use core::arch::asm;

//extern crate alloc;

use drivers::uart::UartWriter;

use crate::{drivers::pci::pci_enable_device, exceptions::install_exception_handlers};

// C functions
unsafe extern "C" {
    fn pci_enable_device_c(base: u64) -> bool;
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_x0: u64, _dtb_ptr: *const u8) -> ! {
    serial_println!("\x1B[1;32m[   INFO    ] Hello World!\x1B[0m");
    serial_println!("\x1B[1;32m[   INFO    ] MVOS aarch64 version 0.0.2\x1B[0m");

    serial_println!("[  SYSTEM   ] Installing exception handlers... ");
    unsafe { install_exception_handlers() };

    let gpu_base = drivers::pci::find_pci_device(0x1af4, 0x1050);

    serial_println!("[  DRIVERS  ] Finding GPU device... {}\x1b[0m", {if gpu_base == 0x0 {"\x1b[0;31mFAILED"} else {"\x1b[0;32mSUCCESS"}});

    unsafe { 
        let gpu_enabled =  pci_enable_device_c(gpu_base); 
        serial_println!("[  DRIVERS  ] Enabling GPU device... {}\x1b[0m", {if gpu_enabled {"\x1b[0;32mSUCCESS"} else {"\x1b[0;31mFAILED"}});
    }

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

#[unsafe(no_mangle)]
pub extern "C" fn c_panic() { panic!("Panic caused in C source."); }

// pub mod framebuffer;
pub mod drivers;
pub mod exceptions;
pub mod memory;
pub mod bindings;