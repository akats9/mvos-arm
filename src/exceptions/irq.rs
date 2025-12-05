use core::arch::asm;

use crate::{memory::mmio::{mmio_read32, mmio_read64, mmio_write32, mmio_write64, mmio_write8}, TIMER};

pub const GICD: usize = 0x08000000;
pub const GICC: usize = 0x08010000;

pub fn gic_init() {
    // Reset
    mmio_write32(GICD as u64 + 0x000, 0);
    for i in (0..0x60) {
        mmio_write32(GICD as u64 + 0x100 + 4*i, 0);
    }
    mmio_write32(GICC as u64 + 0x000, 0);
    mmio_write8(GICC as u64 + 0x004, 0);

    // Enable Distributor
    let mut d = mmio_read32(GICD as u64);
    d |= 1;
    mmio_write32(GICD as u64, d);

    // Enable timer interrupt (id: 30)
    enable_interrupt(30);

    // Enable uart interrupt (id: 33)
    enable_interrupt(33);

    // Set priority mask
    mmio_write8(GICC as u64 + 0x4, 0xff);

    // Enable CPU interface
    let mut d = mmio_read32(GICC as u64);
    d |= 1;
    mmio_write32(GICC as u64, d);

    // Unmask interrupts
    unsafe { asm!("msr daifclr, #2") };
}

pub fn enable_timer() {
    // Read CNTFRQ_EL0
    let cntfrq: usize;
    unsafe { asm!("mrs {}, cntfrq_el0", out(reg) cntfrq); }
    // Calculate TVAL to interrupt every 1 ms
    let tval = cntfrq /1000;
    unsafe { asm!("msr cntp_tval_el0, {}", in(reg) tval); }

    unsafe {
        asm!("mrs x0, cntp_ctl_el0");
        asm!("bic x0, x0, #0b10");
        asm!("orr x0, x0, #0b01");
        asm!("msr cntp_ctl_el0, x0");
    }
}

pub fn tick_timer() {
    unsafe {
        TIMER += 1;
        let cntfrq: usize;
        asm!("mrs {}, cntfrq_el0", out(reg) cntfrq);
        let cval0: usize;
        asm!("mrs {}, cntp_cval_el0", out(reg) cval0);
        let cval = cval0 + (cntfrq/1000);
        asm!("msr cntp_cval_el0, {}", in(reg) cval);
        
    }
}

pub fn enable_interrupt(irq_num: u64) {
    let reg = 0x100 + (irq_num/32)*4;
    let bit = irq_num % 32;
    let mut r = mmio_read32(GICD as u64 + reg);
    r |= 1 << bit;
    mmio_write32(GICD as u64 + reg, r);
} 