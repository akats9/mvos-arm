#ifndef MMU_H_
#define MMU_H_

#define PAGE_SIZE 4096
#define UART_BASE 0x09000000

void mmu_alloc();
void mmu_init();
void mmu_unmap(uint64_t pa, uint64_t va);

#endif