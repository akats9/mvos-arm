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

pub fn mmio_read8(addr: u8) -> u8 {
   unsafe { (addr as *mut u8).read_volatile() as u8 } 
}

pub fn mmio_read16(addr: u16) -> u16 {
   unsafe { (addr as *mut u16).read_volatile() as u16 } 
}

pub fn mmio_read32(addr: u32) -> u32 {
   unsafe { (addr as *mut u32).read_volatile() as u32 } 
}
pub fn mmio_read64(addr: u64) -> u64 {
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

pub fn mmio_write32(reg: u64, data: u32) {
    unsafe {
        (reg as *mut u32).write_volatile(data);
    }
}

pub fn mmio_write64(reg: u64, data: u64) {
    unsafe {
        (reg as *mut u32).write_volatile(data as u32);
    }
}

unsafe extern "C" {
    static heap_bottom: u64;
}

// Thread-safe global allocator state
static NEXT_FREE_MEMORY: AtomicU64 = AtomicU64::new(0x0902_0000_0000);

/// Initialize the allocator (call this once at startup)
pub fn init_allocator() {
    unsafe {
        let stack_addr = core::ptr::addr_of!(heap_bottom) as u64;
        NEXT_FREE_MEMORY.store(stack_addr, Ordering::Relaxed);
    }
}

// Simple page-aligned allocator
pub fn alloc(size: u64) -> u64 {
    // Page align current position (round up to 4KB boundary)
    let current = NEXT_FREE_MEMORY.load(Ordering::Relaxed);
    let aligned_current = (current + 0xFFF) & !0xFFF;
    
    // Page align requested size (round up to 4KB boundary)
    let aligned_size = (size + 0xFFF) & !0xFFF;
    
    // Update next free memory position
    let result = aligned_current;
    NEXT_FREE_MEMORY.store(aligned_current + aligned_size, Ordering::Relaxed);
    
    result
}
