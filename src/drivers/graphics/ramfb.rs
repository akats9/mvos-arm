use core::{ffi::c_char, ptr::null_mut};
use crate::{bootscreen::bootscreen_visual, memory::allocator::alloc_ffi::kmalloc_aligned, mvulkan::MVulkanGPUDriver, serial_println, BPP, SCREENHEIGHT, SCREENWIDTH};

unsafe extern "C" {
    fn ramfb_clear(color: u8, fb_addr: *mut c_char);
    fn ramfb_set_pixel(x: u32, y: u32, r: u8, g: u8, b: u8, fb: *mut c_char);
    fn c_setup_ramfb(fb_addr: *mut c_char, width: u32, height: u32) -> i32;
    fn ramfb_draw_rect(minx: u32, maxx: u32, miny: u32, maxy: u32, r: u8, g: u8, b: u8, fb_addr: *mut c_char);
    fn ramfb_draw_letter(utf8_offset: c_char, r: u8, g: u8, b: u8, x: u32, y: u32, fb_addr: *mut c_char, scale: u8);
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

    fn draw_char(&mut self, utf8: c_char, r: u8, g: u8, b: u8, x: u32, y: u32, scale: u8) {
        unsafe {
            ramfb_draw_letter(utf8, r, g, b, x, y, self.fb_addr, scale);
        }
    }
}