#ifndef RAMFB_H_
#define RAMFB_H_

#include <types.h>
#include <mvos_bindings.h>

#define QEMU_CFG_DMA_CTL_ERROR 0x01
#define QEMU_CFG_DMA_CTL_READ 0x02
#define QEMU_CFG_DMA_CTL_SELECT 0x08
#define QEMU_CFG_DMA_CTL_WRITE 0x10

void qemu_dma_transfer(u32, u32, u64);
void c_setup_ramfb(char*, u32, u32);
void ramfb_clear(u8, char*);
int compare_etc_ramfb(uint8_t*, size_t);

#endif