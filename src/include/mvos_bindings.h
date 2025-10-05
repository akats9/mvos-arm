#include "types.h" 
#include <stdbool.h>

#define BPP 4

#define SCREENHEIGHT 480

#define SCREENWIDTH 640

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

/**
 * dbg FFI binding for C (no varargs)
 */
void c_dbg(const char *message);

/**
 * dgb for integers (bin format, u64)
 */
void c_dbg_bin(uint64_t bin);

/**
 * dgb for integers (hex format, u64)
 */
void c_dgb_hex(uint64_t hex);

void c_panic(const char *msg);

/**
 * serial_println FFI binding for C (no varargs)
 */
void c_serial_println(const char *message);

extern void c_setup_ramfb(char *fb_addr, uint32_t width, uint32_t height);

void interrupt_handler(void);

void kernel_main(uint64_t _x0, const uint8_t *_dtb_ptr);

void kfree(uint8_t *ptr, size_t size);

void kfree_aligned(uint8_t *ptr, size_t size, size_t align);

uint8_t *kmalloc(size_t size);

uint8_t *kmalloc_aligned(size_t size, size_t align);

uint32_t mmio_read32(uint64_t addr);

uint64_t mmio_read64(uint64_t addr);

void mmio_write32(uint64_t reg, uint32_t data);

extern void mmu_init(void);

extern bool pci_enable_device_c(uint64_t base);

uint64_t pci_get_bar(uint64_t base, uint8_t offset, uint8_t index);

extern void ramfb_clear(uint8_t color, char *fb_addr);

void serror_current_el_spx_handler(void);

extern void set_mair(void);

extern void set_paging(void);

extern void set_tcr(void);

void sync_current_el_spx_handler(struct InterruptFrame *frame);

void verify_MMU(void);
