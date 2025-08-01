use core::{arch::asm, ptr::read_volatile};

use crate::{drivers::{dtb_parser::DeviceTreeParser, ramfb::setup_ramfb}, serial_println};

use alloc::alloc::{alloc, Layout};

//pub static mut fb_addr: *mut u8 = 0x0902_0000_0000 as *mut u8;
pub const fb_addr: u64 = 0x40100000;

pub fn gpu_init(dtb_ptr: *const u8) -> Result<(), &'static str> {
    let parser = DeviceTreeParser::new(dtb_ptr)?;

    parser.debug_dtb();

    if let Some((config_addr, config_size)) = parser.find_ramfb() {
        serial_println!("[FRAMEBUFFER] \x1B[0;32mFound RAMFB at 0x{:x}, size 0x{:x}\x1B[0m", config_addr, config_size);

        unsafe { 
            //fb_addr = config_addr as u64; 

            setup_ramfb(0x0902_0000 as *mut u64, 800, 600);
        }

        Ok(())
    } else {
        Err("[FRAMEBUFFER] \x1B[1;31mERROR: RAMFB device not found in device tree\x1B[0m")
    }
}

pub fn allocate_fb() {
    let size = 800*600*4;
    let layout = Layout::from_size_align(size, 4096).unwrap();
    //unsafe { fb_addr = alloc(layout); } 
    serial_println!("[ ALLOCATOR ] Done.")
}

pub unsafe fn clear(color: u8) {

    asm!("dc civac, {}", in(reg) 0x0902_0000 as *mut u32);
    asm!("dsb sy");

    serial_println!("[FRAMEBUFFER] Framebuffer address: 0x{:x}", fb_addr as u8);

    serial_println!("[FRAMEBUFFER] Clearing screen with color: {:x}", color);
    for x in 0..(600*(800*4)) {
        (fb_addr as *mut u8).add(x as usize).write_volatile(color);
        // serial_println!("[FRAMEBUFFER] \x1B[0;33mDEBUG: Wrote value 0x{:x} to offset 0x{:x}\x1B[0m", color, x);
        //serial_println!("[FRAMEBUFFER] \x1B[0;33mDEBUG: Read value back: 0x{:x}\x1b[0m", read_volatile((fb_addr as *mut u32).add(x as usize)));
    }
}