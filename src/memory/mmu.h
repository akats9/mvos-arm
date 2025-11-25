#ifndef MMU_H_
#define MMU_H_

#define PAGE_SIZE 4096
#define UART_BASE 0x09000000
#define GIC_BASE 0x8000000
#define PCI_ECAM_BASE 0x4010000000

#define MAIR_DEVICE_nGnRnE 0b00000000
#define MAIR_NORMAL_NOCACHE 0b10000100
#define MAIR_IDX_DEVICE 0
#define MAIR_IDX_NORMAL 1

#define GRANULE_2MB 0x200000
#define GRANULE_4KB 0x1000

void mmu_alloc();
void mmu_init();
void mmu_unmap(uint64_t pa, uint64_t va);
void mmu_map_4kb(uint64_t va, uint64_t pa, uint64_t attr_index, uint64_t level);

#endif