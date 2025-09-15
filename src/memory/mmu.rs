use core::arch::asm;

use crate::serial_println;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn verify_MMU() {
    let mut sctlr: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr);
    if sctlr & (1 << 0) != 0 {
        serial_println!("[    MMU    ] \x1b[1;32mFinished MMU init.\x1b[0m");
    } else {
        serial_println!("[    MMU    ] \x1b[1;31mMMU init failed.\x1b[0m");
    }
}