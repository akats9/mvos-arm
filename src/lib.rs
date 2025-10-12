#![no_std]
#![no_main]
#![feature(slice_internals)]
#![feature(asm_const)]
#![feature(asm_sym)]

use core::{arch::asm, ffi::{c_char, CStr}, ptr::null_mut};

extern crate alloc;

use drivers::uart::UartWriter;
use alloc::vec::Vec;

use crate::{bootscreen::print_bootscreen, exceptions::set_exception_vectors, memory::allocator::{alloc_ffi::kmalloc_aligned, init_heap}, mvulkan::{console, MVulkanGPUDriver}, random::random_bible_line};

// C functions
unsafe extern "C" {
    fn pci_enable_device_c(base: u64) -> bool;
    fn mmu_init();
    fn ramfb_gradient(fb_addr: *mut c_char);
    fn ramfb_matrix(fb_addr: *mut c_char);
}

// Global constants
pub const SCREENWIDTH: u32 = 1280;
pub const SCREENHEIGHT: u32 = 720;
pub const BPP: u32 = 4;
pub const SCALE: u8 = 1;

// Hardware
static mut GPU_DEVICE: Option<*mut dyn MVulkanGPUDriver> = None;

const BIBLE: &str = include_str!("../Bible.TXT");

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_x0: u64, _dtb_ptr: *const u8) -> ! {
    let mut error_count: u32 = 0;
    print_bootscreen();
    serial_println!("\x1B[1;32m[  ☦️INFO   ] Hello World!\x1B[0m");
    serial_println!("\x1B[1;32m[  ☦️INFO   ] MVOS aarch64 version 0.0.3\x1B[0m");

    
    serial_println!("[ ☦️MEMORY  ] Initializing heap...");
    init_heap();
    
    serial_println!("[ ☦️SYSTEM  ] Installing exception handlers... ");
    unsafe {set_exception_vectors();}
    
    serial_println!("[ ☦️MEMORY  ] Initializing MMU...");
    unsafe { mmu_init(); }
    
    let mut RAMFB_DEVICE = drivers::graphics::ramfb::RamFBDriver::new();
    
    unsafe {
        GPU_DEVICE = Some(&mut RAMFB_DEVICE as *mut dyn MVulkanGPUDriver);
    }
    
    serial_println!("[  DRIVERS  ] Enabling Ramfb device...");
    
    match RAMFB_DEVICE.setup() {
        Ok(()) => {},
        Err(e) => { error_count += 1; serial_println!("[  DRIVERS  ]\x1b[0;31m RamFB {}\x1b[0m", e) } 
    };
    
    match RAMFB_DEVICE.bootscreen() {
        Ok(()) => {},
        Err(e) => { error_count += 1; serial_println!("[  DRIVERS  ]\x1b[0;31m RamFB {}\x1b[0m", e) } 
    };
    
    console_println!("[   INFO   ] Hello World!", ; r:0, g:0xff, b:0);
    console_println!("[   INFO   ] MVOS aarch64 version 0.0.3", ; r:0, g:0xff, b:0);

    console_println!("Γεια σου Κοσμε!", ; r: 255, g: 255, b: 255); // hell

    console_print!("Color test" ; color: 0xffaa55);
    console_println!(" Same line test" ; color: 0x55aaff);

    print_bible();
    
    // let virtio_gpu_base = drivers::pci::find_pci_device(0x1af4, 0x1050);

    // serial_println!("[  DRIVERS  ] Finding VirtIO GPU device... {}\x1b[0m", {if virtio_gpu_base == 0x0 {"\x1b[0;31mFAILED"} else {"\x1b[0;32mSUCCESS"}});

    // unsafe { 
    //     let virtio_gpu_enabled =  pci_enable_device_c(virtio_gpu_base); 
    //     serial_println!("[  DRIVERS  ] Enabling VirtIO GPU device... {}\x1b[0m", {if virtio_gpu_enabled {"\x1b[0;32mSUCCESS"} else {"\x1b[0;31mFAILED"}});
    // }

    if error_count == 0 { 
        serial_println!("[ ☦️SYSTEM  ]\x1b[0;32m All processes succeded.\x1b[0m");
        console_println!("[  SYSTEM  ] All processes succeded.", ; r: 0, g: 255, b: 0);
    } else {
        serial_println!("[ ☦️SYSTEM  ]\x1b[0;31m All processes done ({} failed).\x1b[0m", error_count);
        console_println!("[  SYSTEM  ] All processes done ({} failed).", error_count ; r: 200, g: 0, b: 0);
    }

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("\x1B[1;31m[   PANIC   ] SYSTEM PANICKED: {:?}\x1B[0m", info);
    // unsafe {
    //     asm!("mov x0, #0x09000000");
    //     asm!("mov w1, #0x41");
    //     asm!("strb w1, [x0]"); 
    // }
    loop{}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn c_panic(msg: *const c_char) { panic!("Panic caused in C source: {}.", CStr::from_ptr(msg).to_str().unwrap()); }

pub fn print_bible() {
    let lines: Vec<&str> = BIBLE.lines().collect();
    for line in lines {
        console_println!("{}", line ; color: 0xffaa55);
        for _ in (0..2_usize.pow(28)) {
            unsafe {
                asm!("nop");
            }
        }
    }
}

// pub mod framebuffer;
pub mod drivers;
pub mod exceptions;
pub mod memory;
pub mod bindings;
pub mod bootscreen;
pub mod mvulkan;
pub mod random;