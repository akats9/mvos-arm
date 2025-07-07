use core::arch::asm;
use crate::{memory::paging::paging_idmap_setup, serial_println};

unsafe fn hcr_el2_store(input: u64) {
    asm!("msr hcr_el2, {}", in(reg) input);
    asm!("ret");
}

unsafe fn hcr_el2_load() -> u64 {
    let mut ret;
    asm!("mrs {}, hcr_el2", out(reg) ret);
    ret
}

unsafe fn tcr_el1_store(input: u64) {
    asm!("msr tcr_el1, {}", in(reg) input);
} 

unsafe fn tcr_el1_load() -> u64 {
    let mut ret;
    asm!("mrs {}, tcr_el1", out(reg) ret);
    ret
}

unsafe fn sctlr_el1_store(input: u64) {
    asm!("msr sctlr_el1, {}", in(reg) input);
}

unsafe fn sctlr_el1_load() -> u64 {
    let mut ret;
    asm!("mrs {}, sctlr_el1", out(reg) ret);
    ret
}

unsafe fn id_aa64mmfr0_el1_load() -> u64 {
    let mut ret;
    asm!("mrs {}, id_aa64mmfr0_el1", out(reg) ret);
    ret
}

unsafe fn check_4k_granule_support() -> bool {
    let mut id_aa64mmfr0: u64;
    asm!("mrs {}, id_aa64mmfr0_el1", out(reg) id_aa64mmfr0);
    let tgran4 = (id_aa64mmfr0 >> 28) & ((1 << 4) - 1);
    if tgran4 == 0b0000 {
        serial_println!("[    MMU    ] 4KiB granule supported.");
        return true;
    } else if tgran4 == 0b0001 {
        serial_println!("[    MMU    ] 4KiB granule supports 52-bit input addresses and can describe 52-bit output addresses.");
        return true;
    } else {
        serial_println!("[    MMU    ] \x1b[1;33m4KiB granule is not supported on this system.\x1b[0m");
        return false;
    }
}

unsafe fn configure_tcr_el1_4kb() {
    asm!("mrs x0, tcr_el1");
    asm!("bic x0, x0, #(0x3 << 14)");
    asm!("msr tcr_el1, x0");
}

static mut parange: u64 = 0;

unsafe fn check_max_pa_space() {
    let mut id_aa64mmfr0: u64;
    asm!("mrs {}, id_aa64mmfr0_el1", out(reg) id_aa64mmfr0);
    parange = id_aa64mmfr0 & ((1 << 4) - 1);
    match parange {
        0b0000 => serial_println!("[    MMU    ] Maximum PA range: 32bit; 4GB"),
        0b0001 => serial_println!("[    MMU    ] Maximum PA range: 36bit; 64GB"),
        0b0010 => serial_println!("[    MMU    ] Maximum PA range: 40bit; 1TB"),
        0b0011 => serial_println!("[    MMU    ] Maximum PA range: 42bit; 4TB"),
        0b0100 => serial_println!("[    MMU    ] Maximum PA range: 44bit; 16TB"),
        0b0101 => serial_println!("[    MMU    ] Maximum PA range: 48bit; 256TB"),
        0b0110 => serial_println!("[    MMU    ] Maximum PA range: 52bit; 4PB"),
        0b0111 => serial_println!("[    MMU    ] Maximum PA range: 56bit; 64PB"),
        _ => serial_println!("[    MMU    ] \x1b[1;33mPARange bits did not match any known format.\x1b[0m"),
    };
}

unsafe fn configure_pa_space() {
    asm!("mrs x0, tcr_el1");
    asm!("mov x1, {}", in(reg) parange);
    asm!("bfi x0, x1, #16, #3");
    asm!("msr tcr_el1, x0");
}

unsafe fn write_tosz() {
    asm!("mrs x0, tcr_el1");
    asm!("mov x1, #12");
    asm!("bfi x0, x1, #0, #5");
    asm!("msr tcr_el1, x0");
}

unsafe fn enable_address_translation() {
    asm!("mrs x0, sctlr_el1");
    asm!("mov x1, #1");
    asm!("bfi x0, x1, #0, #1");
    asm!("msr sctlr_el1, x0");
}

unsafe fn use_le() {
    asm!("mrs x0, sctlr_el1");
    asm!("bic x0, x0, #(1 << 25");
    asm!("msr sctlr_el1, x0");
}

// Configure memory type
const MT_NORMAL: u64 = 0x0;
const MT_NORMAL_NO_CACHING: u64 = 0x2;
const MT_DEVICE_NGNRNE: u64 = 0x3;
const MT_DEVICE_NGNRE: u64 = 0x4;

const NORMAL_MEMORY: u64 = 0xff;
const NORMAL_MEMORY_NO_CACHING: u64 = 0x40;
const DEVICE_NGNRNE: u64 = 0x00;
const DEVICE_NGNRE: u64 = 0x04;

fn mair_attr(attr: u64, idx: u64) -> u64 {
    attr << (8*idx)
}

fn memory_cpu_setup() {
    let mair = 
        mair_attr(NORMAL_MEMORY, MT_NORMAL) |
        mair_attr(NORMAL_MEMORY_NO_CACHING, MT_NORMAL_NO_CACHING) |
        mair_attr(DEVICE_NGNRNE, MT_DEVICE_NGNRNE) |
        mair_attr(DEVICE_NGNRE, MT_DEVICE_NGNRE);

    unsafe { 
        asm!("mov x0, {}", in(reg) mair);
        asm!("msr mair_el1, x0");
    }
}

/// Initialize the MMU
pub unsafe fn mmu_init() {
    serial_println!("[    MMU    ] Initializing the MMU...");

    memory_cpu_setup();
    paging_idmap_setup(); 

    let mut tcr = tcr_el1_load();
    let mut hcr = hcr_el2_load();
    let mut sctlr = sctlr_el1_load();
    let mmrf0 = id_aa64mmfr0_el1_load();

    hcr &= !(1 << 34);
    tcr = (tcr & !0xc000) | 0x1000;
    tcr = (tcr & !0x70000) | (mmrf0 & 0xf) << 16;
    tcr = (tcr & 0x1) | 0x100;
    tcr = (tcr & !0x200000) | 0x0;

    sctlr = (sctlr & !0x2000000) | 0x0;
    sctlr = (sctlr & !0x1) | 0x1;

    hcr_el2_store(hcr);
    tcr_el1_store(tcr);
    sctlr_el1_store(sctlr);
}