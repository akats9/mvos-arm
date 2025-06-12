use core::arch::asm;

pub fn mmio_write(reg: u64, data: u32) {
    unsafe {
        (reg as *mut u32).write_volatile(data);
    }
}

pub fn mmio_read(reg: u64) -> u64 {
    unsafe { (reg as *mut u32).read_volatile() as u64 }
}

pub fn mmio_write_barrier() {
    unsafe { asm!("dsb sy", options(nostack)); }
}