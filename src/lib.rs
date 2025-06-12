#![no_std]
#![no_main]

use crate::uart::UartWriter;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    serial_println!("Hello World!");
    serial_println!("MVOS aarch64 version 0.0.1");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

pub mod uart;