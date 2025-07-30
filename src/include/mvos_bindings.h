#include "types.h"

/**
 *Align the given address upwards to given alignment
 */
size_t align_up(size_t addr, size_t align);

void c_panic(void);

void kernel_main(uint64_t _x0, const uint8_t *_dtb_ptr);

uint32_t mmio_read32(uint64_t addr);

uint64_t mmio_read64(uint64_t addr);

void mmio_write32(uint64_t reg, uint32_t data);

extern bool pci_enable_device_c(uint64_t base);

uint64_t pci_get_bar(uint64_t base, uint8_t offset, uint8_t index);
