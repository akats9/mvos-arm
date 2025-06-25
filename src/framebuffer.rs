use core::{arch::asm, ptr::read_volatile};

use crate::{drivers::{dtb_parser::DeviceTreeParser, ramfb::setup_ramfb}, serial_println};

use alloc::alloc::{alloc, Layout};

static mut fb_addr: u64 = 0x0;

pub fn gpu_init(dtb_ptr: *const u8) -> Result<(), &'static str> {
    let parser = DeviceTreeParser::new(dtb_ptr)?;

    parser.debug_dtb();

    if let Some((config_addr, config_size)) = parser.find_ramfb() {
        serial_println!("[FRAMEBUFFER] \x1B[0;32mFound RAMFB at 0x{:x}, size 0x{:x}\x1B[0m", config_addr, config_size);

        unsafe { 
            fb_addr = config_addr as u64; 

            setup_ramfb(0x0902_0000 as *mut u64, 800, 600);
        }

        Ok(())
    } else {
        Err("[FRAMEBUFFER] \x1B[1;31mERROR: RAMFB device not found in device tree\x1B[0m")
    }
}

fn allocate_fb() -> *mut u8 {
    let size = 800*600*4;
    let layout = Layout::from_size_align(size, 4096).unwrap();
    unsafe { alloc(layout) as *mut u8 } 
}

pub unsafe fn clear(color: u8) {

    asm!("dc civac, {}", in(reg) 0x0902_0000 as *mut u32);
    asm!("dsb sy");

    let address = allocate_fb();

    serial_println!("[FRAMEBUFFER] Framebuffer address: 0x{:x}", address as u8);

    serial_println!("[FRAMEBUFFER] Clearing screen with color: {:x}", color);
    for x in 0..(600*(800*8)) {
        address.add(x as usize).write_volatile(color);
        //serial_println!("[FRAMEBUFFER] \x1B[0;33mDEBUG: Wrote value 0x{:x} to offset 0x{:x}\x1B[0m", color, x);
        // serial_println!("[FRAMEBUFFER] \x1B[0;33mDEBUG: Read value back: 0x{:x}\x1b[0m", read_volatile((address as *mut u32).add(x as usize)));
    }
}