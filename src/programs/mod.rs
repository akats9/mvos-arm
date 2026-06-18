use core::arch::asm;

use xmas_elf::{ElfFile, program::Type};

use crate::memory::allocator::alloc_ffi::kmalloc;

/// Loads an ELF binary into memory and prepares it for execution in EL0.
/// Returns (entry_point, user_stack_top)
fn load_user_program(elf_data: &[u8]) -> (u64, u64) {
    let elf = ElfFile::new(elf_data).expect("Failed to parse ELF");

    // === 1. Load all PT_LOAD segments ===
    for program_header in elf.program_iter() {
        if let Ok(Type::Load) = program_header.get_type() {
            let vaddr = program_header.virtual_addr() as usize;
            let mem_size = program_header.mem_size() as usize;
            let file_size = program_header.file_size() as usize;
            let offset = program_header.offset() as usize;

            // Allocate memory for this segment
            let dest = kmalloc(mem_size.max(0x1000));

            unsafe {
                // Copy data from ELF into allocated memory
                core::ptr::copy_nonoverlapping(
                    elf_data.as_ptr().add(offset),
                    dest,
                    file_size,
                );

                // Zero out BSS (uninitialized data)
                if mem_size > file_size {
                    core::ptr::write_bytes(
                        dest.add(file_size),
                        0,
                        mem_size - file_size,
                    );
                }
            }
        }
    }

    // === 2. Allocate stack for the user program ===
    const USER_STACK_SIZE: usize = 64 * 1024; // 64 KB stack
    let stack_base = kmalloc(USER_STACK_SIZE);
    let stack_top = stack_base as u64 + USER_STACK_SIZE as u64;

    // === 3. Return entry point and stack ===
    let entry_point = elf.header.pt2.entry_point();

    (entry_point, stack_top)
}

unsafe fn jump_to_el0(entry_point: u64, user_stack_top: u64) -> ! {
    asm! (
        "msr elr_el1, {entry}",
        "msr spsr_el1, {spsr}", 
        "msr sp_el0, {stack}",
        "eret",
        entry = in(reg) entry_point,
        spsr = in(reg) 0x0,
        stack = in(reg) user_stack_top,
        options(noreturn),
    );
}

pub fn run_user_program(elf: &[u8]) {
    let (entry_point, stack_top) = load_user_program(elf);
    unsafe {
        jump_to_el0(entry_point, stack_top);
    }
}
// PROGRAMS
//static WOLFENSTEIN_ELF: &[u8] = include_bytes!("wolfenstein.elf");