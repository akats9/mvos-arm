#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

extern crate alloc;

use crate::{drivers::ramfb::setup_ramfb, framebuffer::{allocate_fb, fb_addr, gpu_init}, uart::UartWriter};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_x0: u64, dtb_ptr: *const u8) -> ! {
    serial_println!("\x1B[1;32m[   INFO    ] Hello World!\x1B[0m");
    serial_println!("\x1B[1;32m[   INFO    ] MVOS aarch64 version 0.0.2\x1B[0m");

    serial_println!("[  DRIVERS  ] Initializing GPU...");
    // match gpu_init(dtb_ptr) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         serial_println!("{e}");
    //     }
    // };

    allocate_fb();

    unsafe { setup_ramfb(fb_addr as *mut u64, 800, 600); }

    serial_println!("[  DRIVERS  ] GPU Initialized.");

    unsafe { framebuffer::clear(0xFF); }

    serial_println!("[   DEBUG   ] \x1B[0;33mScreen cleared\x1B[0m");
    //gpu_clear(0x00FF00FF);
    loop { unsafe { asm!("hlt #0"); } }
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
pub mod mmio;
pub mod framebuffer;
pub mod drivers;
pub mod allocator;