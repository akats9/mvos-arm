use crate::{GPU_DEVICE, SCALE, SCREENHEIGHT};

pub static mut CURSOR: (u32, u32) = (4,4);

/// Insert a newline by shifting the position of the cursor
/// 
/// If the cursor goes beyond the end of the screen, the screen 
/// is cleared to black and the cursor reset to the top.
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

/// Print to the console with an appended newline.
/// 
/// Format string arguments are fully supported. The string 
/// and its arguments are separated by the color using a `;`.
/// 
/// Color can be selected either as r: u8, g: u8, b: u8
/// or as color: u32 (0xRRGGBB)
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

    ($fmt:expr, $($arg:expr),* ; color: $color:expr) => {
        let formatted = ::alloc::format!($fmt, $($arg),*);
        let r = (($color >> 16) & 0xff) as u8;
        let g = (($color >> 8) & 0xff) as u8;
        let b = ($color & 0xff) as u8;
        for c in formatted.chars() {
            if c == '\n' {
                $crate::mvulkan::console::newline();
            } else {
                let utf8 = c as u32 as usize;
                unsafe {
                    if $crate::mvulkan::console::CURSOR.1 > SCREENWIDTH - 8*SCALE as u32 {
                        $crate::mvulkan::console::newline();
                    }
                    (*GPU_DEVICE.unwrap()).draw_char(utf8, r, g, b, $crate::mvulkan::console::CURSOR.1, $crate::mvulkan::console::CURSOR.0, SCALE);
                    $crate::mvulkan::console::CURSOR.1 += (SCALE*8 + 1) as u32;
                }
            }
        }

        $crate::mvulkan::console::newline();
    };

    ($fmt:expr ; r: $r:expr, g: $g:expr, b: $b:expr) => {
        let formatted = ::alloc::format!($fmt);

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

    ($fmt:expr ; color: $color:expr) => {
        let formatted = ::alloc::format!($fmt);
        let r = (($color >> 16) & 0xff) as u8;
        let g = (($color >> 8) & 0xff) as u8;
        let b = ($color & 0xff) as u8;
        for c in formatted.chars() {
            if c == '\n' {
                $crate::mvulkan::console::newline();
            } else {
                let utf8 = c as u32 as usize;
                unsafe {
                    if $crate::mvulkan::console::CURSOR.1 > SCREENWIDTH - 8*SCALE as u32 {
                        $crate::mvulkan::console::newline();
                    }
                    (*GPU_DEVICE.unwrap()).draw_char(utf8, r, g, b, $crate::mvulkan::console::CURSOR.1, $crate::mvulkan::console::CURSOR.0, SCALE);
                    $crate::mvulkan::console::CURSOR.1 += (SCALE*8 + 1) as u32;
                }
            }
        }

        $crate::mvulkan::console::newline();
    };
}

/// Print to the console.
/// 
/// Format string arguments are fully supported. The string 
/// and its arguments are separated by the color using a `;`.
/// 
/// Color can be selected either as r: u8, g: u8, b: u8
/// or as color: u32 (0xRRGGBB)
#[macro_export]
#[macro_use]
macro_rules! console_print {
    ($fmt:expr, $($arg:expr),* ; r: $r:expr, g: $g:expr, b: $b:expr) => {
        let formatted = ::alloc::format!($fmt);

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
    };

    ($fmt:expr, $($arg:expr),* ; color: $color:expr) => {
        let formatted = ::alloc::format!($fmt, $($arg),*);
        let r = (($color >> 16) & 0xff) as u8;
        let g = (($color >> 8) & 0xff) as u8;
        let b = ($color & 0xff) as u8;
        for c in formatted.chars() {
            if c == '\n' {
                $crate::mvulkan::console::newline();
            } else {
                let utf8 = c as u32 as usize;
                unsafe {
                    if $crate::mvulkan::console::CURSOR.1 > SCREENWIDTH - 8*SCALE as u32 {
                        $crate::mvulkan::console::newline();
                    }
                    (*GPU_DEVICE.unwrap()).draw_char(utf8, r, g, b, $crate::mvulkan::console::CURSOR.1, $crate::mvulkan::console::CURSOR.0, SCALE);
                    $crate::mvulkan::console::CURSOR.1 += (SCALE*8 + 1) as u32;
                }
            }
        }
    };

    ($fmt:expr ; r: $r:expr, g: $g:expr, b: $b:expr) => {
        let formatted = ::alloc::format!($fmt);

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
    };

        ($fmt:expr ; color: $color:expr) => {
        let formatted = ::alloc::format!($fmt);
        let r = (($color >> 16) & 0xff) as u8;
        let g = (($color >> 8) & 0xff) as u8;
        let b = ($color & 0xff) as u8;
        for c in formatted.chars() {
            if c == '\n' {
                $crate::mvulkan::console::newline();
            } else {
                let utf8 = c as u32 as usize;
                unsafe {
                    if $crate::mvulkan::console::CURSOR.1 > SCREENWIDTH - 8*SCALE as u32 {
                        $crate::mvulkan::console::newline();
                    }
                    (*GPU_DEVICE.unwrap()).draw_char(utf8, r, g, b, $crate::mvulkan::console::CURSOR.1, $crate::mvulkan::console::CURSOR.0, SCALE);
                    $crate::mvulkan::console::CURSOR.1 += (SCALE*8 + 1) as u32;
                }
            }
        }
    };
}