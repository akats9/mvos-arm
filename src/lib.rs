#![no_std]
#![no_main]

use crate::{framebuffer::gpu_init, uart::UartWriter};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    serial_println!("Hello World!");
    serial_println!("MVOS aarch64 version 0.0.1");
    serial_println!("Initializing GPU...");
    gpu_init();
    //gpu_clear(0x00FF00FF);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

pub mod uart;
pub mod pci;
pub mod mmio;
pub mod framebuffer;
pub mod drivers;