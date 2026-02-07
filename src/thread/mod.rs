use core::arch::asm;

use crate::TIMER;

pub fn sleep(ms: usize) {
    unsafe {
        let start_timer = TIMER;
        while TIMER < start_timer + ms {
            asm!("nop");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn c_sleep(ms: usize) {
    sleep(ms);
}