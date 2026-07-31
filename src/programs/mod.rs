use core::arch::asm;

use xmas_elf::{ElfFile, program::Type};

use crate::{dbg, memory::allocator::alloc_ffi::{kmalloc, kmalloc_aligned}};

unsafe extern "C" {
    fn mmu_map_4kb_wroot(root: *mut u64, va: u64, pa: u64, attr_index: u64, level: u64);
}
const PAGE_SIZE: usize = 0x1000;

/// Loads an ELF binary into memory and prepares it for execution in EL0.
/// Returns (entry_point, user_stack_top, user_root)
fn load_user_program(elf_data: &[u8]) -> (u64, u64, *mut u64) {
    let elf = ElfFile::new(elf_data).expect("Failed to parse ELF");

    // 1. Create a fresh user page-table root (must be zeroed)
    let user_root = unsafe {
        let ptr = kmalloc_aligned(PAGE_SIZE, PAGE_SIZE) as *mut u64;
        core::ptr::write_bytes(ptr, 0, PAGE_SIZE / 8); // zero the whole page
        ptr
    };

    // 2. Load all PT_LOAD segments
    for program_header in elf.program_iter() {
        if let Ok(Type::Load) = program_header.get_type() {
            let vaddr    = program_header.virtual_addr();
            let mem_size = program_header.mem_size() as usize;
            let file_size = program_header.file_size() as usize;
            let offset   = program_header.offset() as usize;

            // Allocate physical pages (kmalloc currently returns usable memory)
            let phys_base = kmalloc(mem_size.max(PAGE_SIZE)) as u64;

            unsafe {
                // Copy file contents
                core::ptr::copy_nonoverlapping(
                    elf_data.as_ptr().add(offset),
                    phys_base as *mut u8,
                    file_size,
                );

                // Zero BSS
                if mem_size > file_size {
                    core::ptr::write_bytes(
                        (phys_base as *mut u8).add(file_size),
                        0,
                        mem_size - file_size,
                    );
                }
            }

            // Map every page of this segment at the ELF virtual address
            // into the *user* page tables with EL0 permissions (level = 0)
            unsafe {
                let mut virt = vaddr;
                let mut phys = phys_base;
                let end = vaddr + mem_size.max(PAGE_SIZE) as u64;

                while virt < end {
                    // attr_index = 0 (or whatever you use for normal memory)
                    // level = 0 → EL0
                    mmu_map_4kb_wroot(user_root, virt, phys, 0, 0);
                    virt += PAGE_SIZE as u64;
                    phys += PAGE_SIZE as u64;
                }
            }
        }
    }

    // 3. Allocate and map a user stack
    const USER_STACK_SIZE: usize = 64 * 1024;
    const USER_STACK_VADDR: u64 = 0x80000; // pick any free user address

    let stack_phys = kmalloc(USER_STACK_SIZE) as u64;

    unsafe {
        let mut virt = USER_STACK_VADDR;
        let mut phys = stack_phys;
        let end = USER_STACK_VADDR + USER_STACK_SIZE as u64;

        while virt < end {
            mmu_map_4kb_wroot(user_root, virt, phys, 0, 0); // EL0 read/write
            virt += PAGE_SIZE as u64;
            phys += PAGE_SIZE as u64;
        }
    }

    let stack_top = USER_STACK_VADDR + USER_STACK_SIZE as u64;
    let entry_point = elf.header.pt2.entry_point();

    // Return entry point, stack top, and the page-table root
    // (you must keep the root so you can load it into TTBR0_EL1 later)
    (entry_point, stack_top, user_root)
}

unsafe fn jump_to_el0(entry_point: u64, user_stack_top: u64) -> ! {
    asm! (
        "msr elr_el1, {entry}",
        "msr spsr_el1, {spsr}", 
        "msr sp_el0, {stack}",
        "msr daifset, #0xf",
        "eret",
        entry = in(reg) entry_point,
        spsr = in(reg) 0x0,
        stack = in(reg) user_stack_top,
        options(noreturn),
    );
}

unsafe fn mmu_flush_all() {
    // asm!(
    //     "dsb ishst",
    //     "tlbi vmalle1is",
    //     "dsb ish",
    //     "isb",
    // );

    asm!("dsb ishst");
    asm!("tlbi vmalle1");
    asm!("dsb ish");
    asm!("isb");
}

pub fn run_user_program(elf: &[u8]) {
    let (entry_point, stack_top, root) = load_user_program(elf);
    dbg!("loaded user program");
    unsafe {
        asm!("msr ttbr0_el1, {}", in(reg) root as u64);
        dbg!("msr'd");
        mmu_flush_all();
        dbg!("mmu configured; jumping to el0...");
        jump_to_el0(entry_point, stack_top);
    }
}

// PROGRAMS
//static WOLFENSTEIN_ELF: &[u8] = include_bytes!("wolfenstein.elf");
pub static TEST_ELF: &[u8] = include_bytes!("bin/user.elf");