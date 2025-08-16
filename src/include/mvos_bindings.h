#include "types.h"

typedef struct InterruptFrame {
    uint64_t x0;
    uint64_t x1;
    uint64_t x2;
    uint64_t x3;
    uint64_t x4;
    uint64_t x5;
    uint64_t x6;
    uint64_t x7;
    uint64_t x8;
    uint64_t x9;
    uint64_t x10;
    uint64_t x11;
    uint64_t x12;
    uint64_t x13;
    uint64_t x14;
    uint64_t x15;
    uint64_t x16;
    uint64_t x17;
    uint64_t x18;
    uint64_t x29;
    uint64_t x30;
    uint64_t elr;
    uint64_t esr;
    uint64_t far;
} InterruptFrame;

/**
 *Align the given address upwards to given alignment
 */
size_t align_up(size_t addr, size_t align);

void c_panic(const char *msg);

/**
 * FFI binding for C
 */
void c_serial_println(const char *message);

void interrupt_handler(void);

void kernel_main(uint64_t _x0, const uint8_t *_dtb_ptr);

uint32_t mmio_read32(uint64_t addr);

uint64_t mmio_read64(uint64_t addr);

void mmio_write32(uint64_t reg, uint32_t data);

extern void paging_boot(void);

extern bool pci_enable_device_c(uint64_t base);

uint64_t pci_get_bar(uint64_t base, uint8_t offset, uint8_t index);

extern void set_mair(void);

extern void set_paging(void);

extern void set_tcr(void);

void sync_current_el_spx_handler(struct InterruptFrame *frame);
