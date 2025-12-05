use core::{ffi::c_char, ptr::null_mut};
use alloc::vec::Vec;
use spin::mutex;

use crate::{BPP, SCREENHEIGHT, SCREENWIDTH, bootscreen::bootscreen_visual, dbg, memory::allocator::alloc_ffi::kmalloc_aligned, mvulkan::{MVulkanGPUDriver, MVulkanGeometry, MVulkanText}, serial_println, thread};
use crate::{min, max};

pub mod c {
    use core::ffi::c_char;

    unsafe extern "C" {
        pub fn ramfb_clear(color: u8, fb_addr: *mut c_char);
        pub fn ramfb_set_pixel(x: u32, y: u32, r: u8, g: u8, b: u8, fb: *mut c_char);
        pub fn c_setup_ramfb(fb_addr: *mut c_char, width: u32, height: u32) -> i32;
        pub fn ramfb_draw_rect(minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8, fb_addr: *mut c_char);
        pub fn ramfb_draw_letter(utf8_offset: usize, r: u8, g: u8, b: u8, x: u32, y: u32, fb_addr: *mut c_char, scale: u8);
    }
}

/// RamFB device driver that implements MVulkan API.
/// Also includes special functions that are not MVulkan-related.
pub struct RamFBDriver {
    fb_addr: *mut c_char,
}

impl RamFBDriver {
    pub fn new() -> Self {
        Self {
            fb_addr: null_mut(),
        }
    }

    pub fn bootscreen(&mut self) -> Result<(), &'static str>{
        if self.fb_addr == null_mut() {
            return Err("Error: attempted to display bootscreen before RamFB framebuffer allocation.");
        }
        bootscreen_visual(self.fb_addr);
        Ok(())
    }
}

impl MVulkanGPUDriver for RamFBDriver {
    fn setup(&mut self) -> Result<(), &'static str> {
        serial_println!("[ ☦️SYSTEM  ] Allocating Ramfb framebuffer...");
        let fb_addr = kmalloc_aligned((BPP*SCREENWIDTH*SCREENHEIGHT) as usize, 4096);
        self.fb_addr = fb_addr;
        unsafe { 
            let res = c::c_setup_ramfb(self.fb_addr, SCREENWIDTH, SCREENHEIGHT); 
            if res != 0 {
                return Err("Error: failed to initialize RamFB device (device not present).");
            } else {
                return Ok(());
            }
        };
    }

    fn clear(&mut self, color: u8) {
        unsafe {
            c::ramfb_clear(color, self.fb_addr);
        }
    }

    fn draw_rect(&mut self, minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8) {
        unsafe {
            c::ramfb_draw_rect(minx, maxx, miny, maxy, r, g, b, self.fb_addr);
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        unsafe {
            c::ramfb_set_pixel(x, y, r, g, b, self.fb_addr);
        }
    }

    fn draw_char(&mut self, utf8: usize, r: u8, g: u8, b: u8, x: u32, y: u32, scale: u8) {
        unsafe {
            c::ramfb_draw_letter(utf8, r, g, b, x, y, self.fb_addr, scale);
        }
    }

    fn as_geometry(&self) -> Option<&dyn crate::mvulkan::MVulkanGeometry> {
        Some(self)
    }

    fn as_geometry_mut(&mut self) -> Option<&mut dyn crate::mvulkan::MVulkanGeometry> {
        Some(self)
    }

    fn as_text(&self) -> Option<&dyn crate::mvulkan::MVulkanText> {
        Some(self)
    }

    fn as_text_mut(&mut self) -> Option<&mut dyn crate::mvulkan::MVulkanText> {
        Some(self)
    }
}

impl MVulkanGeometry for RamFBDriver {
    fn draw_circle(&mut self, Ox: u32, Oy: u32, R: u32, r: u8, g: u8, b: u8, fill: bool) {
        if Ox < R || Ox + R > SCREENWIDTH || Oy < R || Oy + R > SCREENHEIGHT { return; }
        let mut points: Vec<(u32, u32)> = Vec::new();
        let radius_sq = R.pow(2);

        if fill {
            for dy in -(R as i32)..=R as i32 {
                let y = Oy as i32 + dy;
                let dy_sq = dy.pow(2);

                let rem = radius_sq as i32 - dy_sq;
                if rem < 0 {continue;}

                let dx_max = isqrt(rem);

                for dx in -dx_max..=dx_max {
                    let x = Ox as i32 + dx;
                    points.push((x as u32,y as u32));
                }
            }
        } else {
            let mut x = R;
            let mut y = 0;

            let mut p: i32 = 1 - r as i32;
            while x > y {
                y += 1;
                if p <= 0 { p = p + 2*y as i32 + 1; }
                else { x -= 1; p = p+ 2*y as i32 - 2*x as i32 + 1; }
                if x < y { break; }
                points.push((x as u32 + Ox, y as u32 + Oy));
                points.push(((-(x as i32) + Ox as i32) as u32, y as u32 + Oy));
                points.push((x as u32 + Ox, (-(y as i32) + Oy as i32) as u32));
                points.push(((-(x as i32) + Ox as i32) as u32, (-(y as i32) + Oy as i32) as u32));
                if x != y {
                    points.push((y as u32 + Ox, x as u32 + Oy));
                    points.push(((-(y as i32) + Ox as i32) as u32, x as u32 + Oy));
                    points.push((y as u32 + Ox, (-(x as i32) + Oy as i32) as u32));
                    points.push(((-(y as i32) + Ox as i32) as u32, (-(x as i32) + Oy as i32) as u32));
                }
            }
        }
        

        for point in points {
            let (x, y) = point;
            self.set_pixel(x, y, r, g, b);
        }
    }

    fn draw_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, r: u8, g: u8, b: u8, fill: bool) {
        if fill {
            // Sort vertices by y-coordinate (y1 <= y2 <= y3)
            let mut vertices = [(x1, y1), (x2, y2), (x3, y3)];
            vertices.sort_by_key(|v| v.1);
            let [(x1, y1), (x2, y2), (x3, y3)] = vertices;

            // Helper function to interpolate x coordinate for a given y
            let interpolate_x = |y: u32, ya: u32, xa: u32, yb: u32, xb: u32| -> u32 {
                if yb == ya {
                    return xa;
                }
                let ya = ya as i32;
                let yb = yb as i32;
                let xa = xa as i32;
                let xb = xb as i32;
                let y = y as i32;
                
                (xa + (xb - xa) * (y - ya) / (yb - ya)) as u32
            };

            // Fill the triangle by splitting it into two parts
            // Top part: from y1 to y2
            for y in y1..=y2 {
                let x_left = interpolate_x(y, y1, x1, y3, x3);
                let x_right = interpolate_x(y, y1, x1, y2, x2);
                
                let (x_start, x_end) = if x_left <= x_right {
                    (x_left, x_right)
                } else {
                    (x_right, x_left)
                };
                
                for x in x_start..=x_end {
                    self.set_pixel(x, y, r, g, b);
                }
            }

            // Bottom part: from y2 to y3
            for y in (y2 + 1)..=y3 {
                let x_left = interpolate_x(y, y1, x1, y3, x3);
                let x_right = interpolate_x(y, y2, x2, y3, x3);
                
                let (x_start, x_end) = if x_left <= x_right {
                    (x_left, x_right)
                } else {
                    (x_right, x_left)
                };
                
                for x in x_start..=x_end {
                    self.set_pixel(x, y, r, g, b);
                }
            }
        } else {
            self.draw_line(x1, y1, x2, y2, r, g, b);
            self.draw_line(x2, y2, x3, y3, r, g, b);
            self.draw_line(x3, y3, x1, y1, r, g, b);
        }
    }

    fn draw_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, r: u8, g: u8, b: u8) {
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.set_pixel(x0 as u32, y0 as u32, r, g, b);
            
            if x0 == x1 && y0 == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

impl MVulkanText for RamFBDriver {
    fn draw_textbox(&mut self, message: &str, x: u32, y: u32, scale: u8, color: u32) {
        if x > SCREENWIDTH - 8 || y > SCREENHEIGHT - 8 {return;}
        let mut cursor: (u32, u32) = (x,y);
        for c in message.chars() {
            if c == '\n' {
                newline(&mut cursor, x, y, scale);
            } else {
                let utf8 = c as u32 as usize;
                let r = (color >> 16 & 0xff) as u8;
                let g = (color >> 8 & 0xff) as u8;
                let b = (color & 0xff) as u8;
                self.draw_char(utf8, r, g, b, cursor.0, cursor.1, scale);
                cursor.0 += (scale*8) as u32;
            }
        }
    }
}

fn newline(cursor: &mut (u32, u32), x: u32, y: u32, scale: u8) {
    dbg!("NEWLINE");
    cursor.0 = x;
    cursor.1 += (scale*8) as u32;
}

fn isqrt(n: i32) -> i32 {
    if n < 0 { return 0; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + n/x) / 2;
    }

    x
}

/// Find maximum of values
#[macro_export]
#[macro_use]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}

/// Find minimum of values
#[macro_export]
#[macro_use]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}