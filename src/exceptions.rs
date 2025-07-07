// Even simpler - use a macro to generate the vector table
core::arch::global_asm!(
    r#"
    .balign 2048
    .global exception_vector_table
    exception_vector_table:
    // Skip to offset 0x200
    .skip 0x200
    
    // Sync exception handler at 0x200
    mrs x0, elr_el1
    add x0, x0, #4      // Skip faulting instruction  
    msr elr_el1, x0
    eret
    
    // Pad to 128 bytes
    .balign 128
    "#
);

unsafe extern "C" {
    static exception_vector_table: u8;
}

pub unsafe fn install_exception_handlers() {
    let addr = &exception_vector_table as *const _ as u64;
    core::arch::asm!("msr vbar_el1, {}", in(reg) addr);
    core::arch::asm!("isb");
}