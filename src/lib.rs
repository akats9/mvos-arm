#![no_std]
#![no_main]
#![feature(slice_internals)]

use core::{arch::asm, panic::PanicInfo};
use crate::drivers::uart::UartWriter;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    serial_println!("[  SYSTEM  ] \x1b[1;32mHello world!\x1b[0m");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

pub mod drivers;