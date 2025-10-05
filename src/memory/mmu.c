#include <mvos_bindings.h>
#include <types.h>
#include "../drivers/pci.h"
#include "mmu.h"

#define MAIR_DEVICE_nGnRnE 0b00000000
#define MAIR_NORMAL_NOCACHE 0b10000100
#define MAIR_IDX_DEVICE 0
#define MAIR_IDX_NORMAL 1

#define PD_TABLE 0b11
#define PD_BLOCK 0b01

#define UXN_BIT 54
#define PXN_BIT 53
#define AF_BIT 10
#define SH_BIT 8
#define AP_BIT 6
#define MAIR_BIT 2

#define PAGE_TABLE_ENTRIES 512

#define ENTRY_MASK 0xfffffffff000ULL

#define GRANULE_2MB 0x200000
#define GRANULE_4KB 0x1000

static uint64_t kernel_start = 0x40000000; // ☦️
static uint64_t kcode_end    = 0x50000000;

uint64_t page_table_l0[PAGE_TABLE_ENTRIES] __attribute__((aligned(PAGE_SIZE)));

// void mmu_map_2mb(uint64_t va, uint64_t pa, uint64_t attr_index) {
//     uint64_t l0_index = (va >> 39) & 0x1ff;
//     uint64_t l1_index = (va >> 30) & 0x1ff;
//     uint64_t l2_index = (va >> 21) & 0x1ff;

//     if (!(page_table_l0[l0_index] & 1)) {
//         uint64_t* l1 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
//         page_table_l0[l0_index] = ((uint64_t)l1 & ENTRY_MASK) | PD_TABLE;

//         if (!(l1[l1_index] & 1)) {
//             uint64_t* l2 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
//             l1[l1_index] = ((uint64_t)l2 & ENTRY_MASK) | PD_TABLE;
//         }

//         uint64_t* l2 = (uint64_t*)(l1[l1_index] & ENTRY_MASK);
//         uint64_t attr = ((uint64_t)1 << UXN_BIT) | ((uint64_t)0 << PXN_BIT) | (1 << AF_BIT) | (0b11 << SH_BIT) | (attr_index << MAIR_BIT) | PD_BLOCK;
//         l2[l2_index] = (pa & ENTRY_MASK) | attr;
//     }
// }

void* memset(void* ptr, int value, size_t len) {
    long int dstp = (long int)ptr;

    if (len >= 8) {
        size_t xlen;
        op_t cccc;

        cccc = (unsigned char)value;
        cccc |= cccc << 8;
        cccc |= cccc << 16;
        if (OPSIZ > 4) cccc |= (cccc << 16) << 16;

        while (dstp % OPSIZ != 0) {
            ((byte *) dstp)[0] = value;
            dstp++;
            len--;
        }

        xlen = len / (OPSIZ * 8);
        while (xlen > 0) {
            ((op_t*) dstp)[0] = cccc;
            ((op_t*) dstp)[1] = cccc;
            ((op_t*) dstp)[2] = cccc;
            ((op_t*) dstp)[3] = cccc;
            ((op_t*) dstp)[4] = cccc;
            ((op_t*) dstp)[5] = cccc;
            ((op_t*) dstp)[6] = cccc;
            ((op_t*) dstp)[7] = cccc;
            dstp += 8 * OPSIZ;
            xlen--;
        }

        len %= OPSIZ * 8;

        xlen = len / OPSIZ;
        while (xlen > 0) {
            ((op_t*) dstp)[0] = cccc;
            dstp += OPSIZ;
            xlen--;
        }
        len %= OPSIZ;
    }
    while (len > 0) {
        ((byte*) dstp)[0] = value;
        dstp++;
        len--;
    }

    return ptr;
}

void mmu_map_2mb(uint64_t va, uint64_t pa, uint64_t attr_index) {
    uint64_t l0_index = (va >> 39) & 0x1ff;
    uint64_t l1_index = (va >> 30) & 0x1ff;
    uint64_t l2_index = (va >> 21) & 0x1ff;

    // Get or create L1
    uint64_t* l1;
    if (!(page_table_l0[l0_index] & 1)) {
        l1 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
        memset(l1, 0, PAGE_SIZE);
        page_table_l0[l0_index] = ((uint64_t)l1 & ENTRY_MASK) | PD_TABLE;
    } else {
        l1 = (uint64_t*)(page_table_l0[l0_index] & ENTRY_MASK);
    }

    // Get or create L2
    uint64_t* l2;
    if (!(l1[l1_index] & 1)) {
        l2 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
        memset(l2, 0, PAGE_SIZE);
        l1[l1_index] = ((uint64_t)l2 & ENTRY_MASK) | PD_TABLE;
    } else {
        l2 = (uint64_t*)(l1[l1_index] & ENTRY_MASK);
    }

    // Create mapping
    uint64_t attr = ((uint64_t)1 << UXN_BIT) | ((uint64_t)0 << PXN_BIT) | 
                    (1 << AF_BIT) | (0b11 << SH_BIT) | 
                    (attr_index << MAIR_BIT) | PD_BLOCK;
    l2[l2_index] = (pa & ENTRY_MASK) | attr;
}

/** Level 0 = EL0, Level 1 = EL1, Level 2 = Shared */
void mmu_map_4kb(uint64_t va, uint64_t pa, uint64_t attr_index, uint64_t level) {
    uint64_t l0_index = (va >> 39) & 0x1ff, l1_index = (va >> 30) & 0x1ff, l2_index = (va >> 21) & 0x1ff, l3_index = (va >> 12) & 0x1ff;

    if (!(page_table_l0[l0_index] & 1)) {
        uint64_t* l1 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
        page_table_l0[l0_index] = ((uint64_t)l1 & ENTRY_MASK) | PD_TABLE;
    }

    uint64_t* l1 = (uint64_t*)(page_table_l0[l0_index] & ENTRY_MASK);
    if (!(l1[l1_index] & 1)) {
        uint64_t* l2 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
        l1[l1_index] = ((uint64_t)l2 & ENTRY_MASK) | PD_TABLE;
    }

    uint64_t* l2 = (uint64_t*)(l1[l1_index] & ENTRY_MASK);
    uint64_t l2_val = l2[l2_index];
    if (!(l2_val & 1)) {
        uint64_t* l3 = (uint64_t*)kmalloc_aligned(PAGE_SIZE, PAGE_SIZE);
        l2[l2_index] = ((uint64_t)l3 & ENTRY_MASK) | PD_TABLE;
    } else if ((l2_val & 0b11) == PD_BLOCK) {
        return;
    }

    uint64_t* l3 = (uint64_t*)(l2[l2_index] & ENTRY_MASK);
    if (l3[l3_index] & 1) {
        return;
    }

    uint8_t permission = 0;
    switch (level) {
        case 0: permission = 0b01; break;
        case 1: permission = 0b00; break;
        case 2: permission = 0b10; break;
        default: break;
    }

    uint64_t attr = ((uint64_t)(level == 1) << UXN_BIT) | ((uint64_t)0 << PXN_BIT) | (1 << AF_BIT) | (0b01 << SH_BIT) | (permission << AP_BIT) | (attr_index < MAIR_BIT) | 0b11;
    l3[l3_index] = (pa & ENTRY_MASK) | attr;
}

static inline void mmu_flush_all() {
    asm volatile (
        "dsb ishst\n"
        "tlbi vmalle1is\n"
        "dsb ish\n"
        "isb\n"
    );
}

static inline void mmu_flush_icache() {
    asm volatile (
        "ic iallu\n"
        "isb\n"
    );
}

void mmu_unmap(uint64_t pa, uint64_t va) {
    uint64_t l0_index = (va >> 39) & 0x1ff, l1_index = (va >> 30) & 0x1ff, l2_index = (va >> 21) & 0x1ff, l3_index = (va >> 12) & 0x1ff;

    if (!(page_table_l0[l0_index] & 1)) return;

    uint64_t* l1 = (uint64_t*)(page_table_l0[l0_index] & ENTRY_MASK);
    if (!(l1[l1_index] & 1)) return;

    uint64_t* l2 = (uint64_t*)(l1[l1_index] & ENTRY_MASK);
    uint64_t l3_val = l2[l2_index];
    if (!(l3_val & 1)) return;
    else if (!(l3_val & 0b11) == PD_BLOCK) {
        l2[l2_index] = 0;
        return;
    }

    uint64_t* l3 = (uint64_t*)(l2[l2_index] & ENTRY_MASK);
    l3[l3_index] = 0;

    mmu_flush_all();
    mmu_flush_icache();
}

void mmu_init() {
    uint64_t kstart = kernel_start;
    uint64_t kend = kcode_end;

    for (uint64_t addr = kstart; addr <= kend; addr += GRANULE_2MB) mmu_map_2mb(addr, addr, MAIR_IDX_NORMAL);
    for (uint64_t addr = UART_BASE - 0x1000; addr <= UART_BASE + 0x100000; addr += GRANULE_4KB) mmu_map_4kb(addr, addr, MAIR_IDX_DEVICE, 1);

    uint64_t mair = (MAIR_DEVICE_nGnRnE << (MAIR_IDX_DEVICE * 8)) | (MAIR_NORMAL_NOCACHE << (MAIR_IDX_NORMAL * 8));
    asm volatile ("msr mair_el1, %0" :: "r"(mair));

    uint64_t tcr = ((64-48) << 0) | ((64-48) << 16) | (0b00 << 14) | (0b10 << 30);
    asm volatile ("msr tcr_el1, %0" :: "r"(tcr));

    asm volatile ("dsb ish\n" "isb");
    asm volatile ("msr ttbr0_el1, %0" :: "r"(page_table_l0));

    u64 sctlr_el1;
    asm volatile ("mrs %0, sctlr_el1\n" : "=r"(sctlr_el1));
    // asm volatile ("orr x0, x0, #0x1\n");
    // asm volatile ("bic x0, x0, #(1 << 19)\n");
    c_dbg("sctlr_el1 before:");
    c_dbg_bin(sctlr_el1);
    sctlr_el1 |= 0x1;
    c_dbg("sctlr_el1 after:");
    c_dbg_bin(sctlr_el1);
    c_dbg("☦️");
    asm volatile ("msr sctlr_el1, %0\n" :: "r"(sctlr_el1));
    asm volatile ("isb\n");
    asm volatile ("nop");

    verify_MMU();
}