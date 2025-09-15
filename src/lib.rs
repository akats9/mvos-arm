#![no_std]
#![no_main]
#![feature(slice_internals)]
#![feature(asm_const)]
#![feature(asm_sym)]

use core::{arch::asm, ffi::{c_char, CStr}};

//extern crate alloc;

use drivers::uart::UartWriter;

use crate::{bootscreen::print_bootscreen, drivers::graphics::ramfb::setup_ramfb, exceptions::set_exception_vectors, memory::allocator::{alloc_ffi::kmalloc, init_heap}};

// C functions
unsafe extern "C" {
    fn pci_enable_device_c(base: u64) -> bool;
    fn mmu_init();
    fn c_setup_ramfb(fb_addr: *mut c_char, width: u32, height: u32);
    fn ramfb_clear(color: u8, fb_addr: *mut c_char);
}

// Assembly functions 
unsafe extern "C" {
    fn set_tcr();
    fn set_mair();
    fn set_paging();
}

// Global constants
pub const SCREENWIDTH: u32 = 640;
pub const SCREENHEIGHT: u32 = 480;
pub const BPP: u32 = 4;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_x0: u64, _dtb_ptr: *const u8) -> ! {
    print_bootscreen();
    serial_println!("\x1B[1;32m[  ☦️INFO   ] Hello World!\x1B[0m");
    serial_println!("\x1B[1;32m[  ☦️INFO   ] MVOS aarch64 version 0.0.2\x1B[0m");

    serial_println!("[ ☦️MEMORY  ] Initializing heap...");
    init_heap();

    serial_println!("[ ☦️SYSTEM  ] Installing exception handlers... ");
    unsafe {set_exception_vectors();}

    serial_println!("[ ☦️MEMORY  ] Initializing MMU...");
    unsafe { mmu_init(); }

    serial_println!("[ ☦️SYSTEM  ] Allocating Ramfb framebuffer...");
    let fb_addr = kmalloc((BPP*SCREENWIDTH*SCREENHEIGHT) as usize);

    serial_println!("[  DRIVERS  ] Enabling Ramfb device...");
    unsafe { c_setup_ramfb(fb_addr, SCREENWIDTH, SCREENHEIGHT);}
    //setup_ramfb(fb_addr as *mut u64, SCREENWIDTH, SCREENHEIGHT);

    let virtio_gpu_base = drivers::pci::find_pci_device(0x1af4, 0x1050);

    serial_println!("[  DRIVERS  ] Finding VirtIO GPU device... {}\x1b[0m", {if virtio_gpu_base == 0x0 {"\x1b[0;31mFAILED"} else {"\x1b[0;32mSUCCESS"}});

    unsafe { 
        let virtio_gpu_enabled =  pci_enable_device_c(virtio_gpu_base); 
        serial_println!("[  DRIVERS  ] Enabling VirtIO GPU device... {}\x1b[0m", {if virtio_gpu_enabled {"\x1b[0;32mSUCCESS"} else {"\x1b[0;31mFAILED"}});
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

// pub mod framebuffer;
pub mod drivers;
pub mod exceptions;
pub mod memory;
pub mod bindings;
pub mod bootscreen;
