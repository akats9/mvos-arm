use crate::{GPU_DEVICE, SCALE, SCREENHEIGHT};

pub static mut CURSOR: (u32, u32) = (4,4);

pub fn newline() {
    unsafe {
        CURSOR.0 += (SCALE*8 + 1) as u32; // LF
        CURSOR.1 = 4; // CR
        if CURSOR.0 > (SCREENHEIGHT - 8*SCALE as u32) as u32 {
            (*GPU_DEVICE.unwrap()).clear(0x00);
            CURSOR.0 = 4;
            CURSOR.1 = 4;
        }
    }
}

#[macro_export]
#[macro_use]
macro_rules! console_println {
    ($fmt:expr, $($arg:expr),* ; r: $r:expr, g: $g:expr, b: $b:expr) => {
        let formatted = ::alloc::format!($fmt, $($arg),*);

        for c in formatted.chars() {
            if c == '\n' {
                $crate::mvulkan::console::newline();
            } else {
                let utf8 = c as u32 as usize;
                unsafe {
                    if $crate::mvulkan::console::CURSOR.1 > SCREENWIDTH - 8*SCALE as u32 {
                        $crate::mvulkan::console::newline();
                    }
                    (*GPU_DEVICE.unwrap()).draw_char(utf8, $r, $g, $b, $crate::mvulkan::console::CURSOR.1, $crate::mvulkan::console::CURSOR.0, SCALE);
                    $crate::mvulkan::console::CURSOR.1 += (SCALE*8 + 1) as u32;
                }
            }
        }

        $crate::mvulkan::console::newline();
    };
}