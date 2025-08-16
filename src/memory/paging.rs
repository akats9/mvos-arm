//! Helper functions for the main paging implementation (C source)

use core::arch::asm;

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn load_config_tcr_el1(config: usize) {
//     asm!("ldr x0, ={}", in(reg) config);
//     asm!("msr tcr_el1, x0");
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn load_config_mair_el1(config: usize) {
//     asm!("ldr x0, ={}", in(reg) config);
//     asm!("msr mair_el1, x0");
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn identity_paging_setup(boot_pgd_attr: usize, boot_pud_attr: usize) {
//     asm!("mov x0, #0"); // PGD's page frame at 0x0
//     asm!("mov x1, #0x1000"); // PUD's page frame at 0x1000

//     asm!("ldr x2, ={}", in(reg) boot_pgd_attr);
//     asm!("orr x2, x1, x2"); // Combine the physical address of next level page with attr
//     asm!("str x2, [x0]");

//     asm!("ldr x2, ={}", in(reg) boot_pud_attr);
//     asm!("mov x3, #0x00000000");
//     asm!("orr x3, x2, x3");
//     asm!("str x3, [x1]"); // 1st 1GB mapped by the 1st entry of PUD
//     asm!("mov x3, #0x40000000");
//     asm!("orr x3, x2, x3");
//     asm!("str x3, [x1, #8]"); // 2nd 1GB mapped by the 2nd entry of PUD

//     asm!("msr ttbr0_el1, x0"); // Load PGD to the bottom translation based register
    
//     asm!("mrs x2, sctlr_el1");
//     asm!("orr x2, x2, #1");
//     asm!("msr sctlr_el1, x2"); // Enable MMU, cache remains disabled
// }