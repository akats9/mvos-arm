use core::arch::asm;

use crate::TIMER;

pub fn sleep(secs: usize) {
    unsafe {
        let start_timer = TIMER;
        while TIMER != start_timer + secs {
            asm!("nop");
        }
    }
}