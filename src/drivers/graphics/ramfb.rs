use core::{ffi::c_char, ptr::null_mut};
use alloc::vec::Vec;

use crate::{bootscreen::bootscreen_visual, memory::allocator::alloc_ffi::kmalloc_aligned, mvulkan::{MVulkanGPUDriver, MVulkanGeometry}, serial_println, BPP, SCREENHEIGHT, SCREENWIDTH};
use crate::{min, max};

unsafe extern "C" {
    fn ramfb_clear(color: u8, fb_addr: *mut c_char);
    fn ramfb_set_pixel(x: u32, y: u32, r: u8, g: u8, b: u8, fb: *mut c_char);
    fn c_setup_ramfb(fb_addr: *mut c_char, width: u32, height: u32) -> i32;
    fn ramfb_draw_rect(minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8, fb_addr: *mut c_char);
    fn ramfb_draw_letter(utf8_offset: usize, r: u8, g: u8, b: u8, x: u32, y: u32, fb_addr: *mut c_char, scale: u8);
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
            let res = c_setup_ramfb(self.fb_addr, SCREENWIDTH, SCREENHEIGHT); 
            if res != 0 {
                return Err("Error: failed to initialize RamFB device (device not present).");
            } else {
                return Ok(());
            }
        };
    }

    fn clear(&mut self, color: u8) {
        unsafe {
            ramfb_clear(color, self.fb_addr);
        }
    }

    fn draw_rect(&mut self, minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8) {
        unsafe {
            ramfb_draw_rect(minx, maxx, miny, maxy, r, g, b, self.fb_addr);
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        unsafe {
            ramfb_set_pixel(x, y, r, g, b, self.fb_addr);
        }
    }

    fn draw_char(&mut self, utf8: usize, r: u8, g: u8, b: u8, x: u32, y: u32, scale: u8) {
        unsafe {
            ramfb_draw_letter(utf8, r, g, b, x, y, self.fb_addr, scale);
        }
    }

    fn as_geometry(&self) -> Option<&dyn crate::mvulkan::MVulkanGeometry> {
        Some(self)
    }

    fn as_geometry_mut(&mut self) -> Option<&mut dyn crate::mvulkan::MVulkanGeometry> {
        Some(self)
    }
}

impl MVulkanGeometry for RamFBDriver {
    fn draw_circle(&mut self, Ox: u32, Oy: u32, R: u32, r: u8, g: u8, b: u8) {
        if Ox < R || Ox + R > SCREENWIDTH || Oy < R || Oy + R > SCREENHEIGHT { return; }
        let mut points: Vec<(u32, u32)> = Vec::new();
        let radius_sq = R.pow(2);

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

        for point in points {
            let (x, y) = point;
            self.set_pixel(x, y, r, g, b);
        }
    }

    fn draw_triangle(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, r: u8, g: u8, b: u8) {
        let miny = min!(y1, y2, y3);
        let maxy = max!(x1, x2, x3);

        for y in miny..=maxy {
            let mut intersection_x: [i32; 2] = [-1; 2];
            if y1 <= y && y <= y2 {
                let x = x1 + (y - y1)*(x2 - x1)/(y2-y1);
                if intersection_x[0] > -1 { if intersection_x[1] > -1 {return;} else {intersection_x[1] = x as i32;}} else {intersection_x[0] = x as i32;}
            } else if y2 <= y && y <= y1 {
                let x = x2 + (y - y2)*(x1 - x2)/(y1-y2);
                if intersection_x[0] > -1 { if intersection_x[1] > -1 {return;} else {intersection_x[1] = x as i32;}} else {intersection_x[0] = x as i32;}
            }
            if y2 <= y && y <= y3 {
                let x = x2 + (y - y2)*(x3 - x2)/(y3-y2);
                if intersection_x[0] > -1 { if intersection_x[1] > -1 {return;} else {intersection_x[1] = x as i32;}} else {intersection_x[0] = x as i32;}
            } else if y3 <= y && y <= y2 {
                let x = x3 + (y - y3)*(x2 - x3)/(y2-y3);
                if intersection_x[0] > -1 { if intersection_x[1] > -1 {return;} else {intersection_x[1] = x as i32;}} else {intersection_x[0] = x as i32;}
            }
            if y3 <= y && y <= y1 {
                let x = x3 + (y - y3)*(x1 - x3)/(y1-y3);
                if intersection_x[0] > -1 { if intersection_x[1] > -1 {return;} else {intersection_x[1] = x as i32;}} else {intersection_x[0] = x as i32;}
            } else if y1 <= y && y <= y3 {
                let x = x1 + (y - y1)*(x3 - x1)/(y3-y1);
                if intersection_x[0] > -1 { if intersection_x[1] > -1 {return;} else {intersection_x[1] = x as i32;}} else {intersection_x[0] = x as i32;}
            }
            intersection_x.sort();
            if intersection_x == [-1; 2] {
                continue;
            } else if intersection_x[0] < 0 && intersection_x[1] >= 0 {
                self.set_pixel(intersection_x[1] as u32, y, r, g, b);
            } else {
                for x in intersection_x[0]..=intersection_x[1] {
                    self.set_pixel(x as u32, y, r, g, b);
                }
            }
        }
    }
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