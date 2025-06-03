#![no_std]
#![no_main]

pub const UART0: *mut u32 = 0x09000000 as *mut u32;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        UART0.write_volatile('A' as u32)
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}