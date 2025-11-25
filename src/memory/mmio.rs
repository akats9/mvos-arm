use core::{arch::asm, ops::Add};
use core::sync::atomic::{AtomicU64, Ordering};

pub fn mmio_write(reg: u64, data: u32) {
    mmio_write32(reg, data);
}

pub fn mmio_read(reg: u64) -> u64 {
    mmio_read64(reg)
}

pub fn mmio_write_barrier() {
    unsafe { asm!("dsb sy", options(nostack)); }
}

#[unsafe(no_mangle)]
pub extern "C" fn mmio_read8(addr: u8) -> u8 {
   unsafe { (addr as *mut u8).read_volatile() as u8 } 
}

pub fn mmio_read16(addr: u16) -> u16 {
   unsafe { (addr as *mut u16).read_volatile() as u16 } 
}

#[unsafe(no_mangle)]
pub extern "C" fn mmio_read32(addr: u64) -> u32 {
   unsafe { (addr as *mut u32).read_volatile() as u32 } 
}

#[unsafe(no_mangle)]
pub extern "C" fn mmio_read64(addr: u64) -> u64 {
   unsafe { (addr as *mut u64).read_volatile() as u64 } 
}

pub fn mmio_write8(reg: u64, data: u8) {
    unsafe {
        (reg as *mut u32).write_volatile(data as u32);
    }
}

pub fn mmio_write16(reg: u64, data: u16) {
    unsafe {
        (reg as *mut u32).write_volatile(data as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mmio_write32(reg: u64, data: u32) {
    unsafe {
        (reg as *mut u32).write_volatile(data);
    }
}

pub fn mmio_write64(reg: u64, data: u64) {
    unsafe {
        (reg as *mut u32).write_volatile(data as u32);
    }
}


