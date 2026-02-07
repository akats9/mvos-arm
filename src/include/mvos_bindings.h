#include "types.h" 
#include <stdbool.h>

#define BLACK 0

#define BLUE 170

#define BPP 4

#define BROWN 11162880

#define CYAN 43690

#define DBG_YELLOW 14402339

#define D_GRAY 5592405

#define ERROR_RED 14483456

#define FAIL_RED 11141120

#define GENERIC_WHITE 16777215

#define GICC 134283264

#define GICD 134217728

#define GREEN 43520

#define INFO_GREEN 4308232

#define L_BLUE 5592575

#define L_CYAN 5636095

#define L_GRAY 11184810

#define L_GREEN 5635925

#define L_MAGENTA 16733695

#define L_RED 16733525

#define MAGENTA 11141290

#define PANIC_RED 16711680

#define RED 11141120

#define SCALE 1

#define SCREENHEIGHT 720

#define SCREENWIDTH 1280

#define SUCCESS_GREEN 65280

#define WARNING_ORANGE 15900431

#define WHITE 16777215

#define YELLOW 16777045

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

extern int32_t c_init_xhci(void);

void c_panic(const char *msg);

/**
 * serial_println FFI binding for C (no varargs)
 */
void c_serial_println(const char *message);

extern int32_t c_setup_ramfb(char *fb_addr, uint32_t width, uint32_t height);

void c_sleep(size_t ms);

extern void display_bootscreen(char *fb_addr);

uint64_t find_pci_device(uint32_t vendor_id, uint32_t device_id);

void interrupt_handler(void);

void kernel_main(uint64_t _x0, const uint8_t *_dtb_ptr);

void kfree(uint8_t *ptr, size_t size);

void kfree_aligned(uint8_t *ptr, size_t size, size_t align);

uint8_t *kmalloc(size_t size);

uint8_t *kmalloc_aligned(size_t size, size_t align);

uint32_t mmio_read32(uint64_t addr);

uint64_t mmio_read64(uint64_t addr);

uint8_t mmio_read8(uint8_t addr);

void mmio_write32(uint64_t reg, uint32_t data);

extern void mmu_init(void);

extern bool pci_enable_device_c(uint64_t base);

extern bool pci_enable_device_c(uint64_t base);

uint64_t pci_get_bar(uint64_t base, uint8_t offset, uint8_t index);

uint64_t pci_setup_bar(uint64_t pci_addr,
                       uint32_t bar_index,
                       uint64_t *mmio_start,
                       uint64_t *mmio_size);

extern void ramfb_clear(uint8_t color, char *fb_addr);

extern void ramfb_draw_letter(size_t utf8_offset,
                              uint8_t r,
                              uint8_t g,
                              uint8_t b,
                              uint32_t x,
                              uint32_t y,
                              char *fb_addr,
                              uint8_t scale);

extern void ramfb_draw_rect(uint32_t minx,
                            uint32_t maxx,
                            uint32_t miny,
                            uint32_t maxy,
                            uint8_t r,
                            uint8_t g,
                            uint8_t b,
                            char *fb_addr);

extern void ramfb_gradient(char *fb_addr);

extern void ramfb_matrix(char *fb_addr);

extern void ramfb_set_pixel(uint32_t x, uint32_t y, uint8_t r, uint8_t g, uint8_t b, char *fb);

void serror_current_el_spx_handler(void);

void sync_current_el_spx_handler(struct InterruptFrame *frame);

void verify_MMU(void);

extern int32_t virtio_generic_setup_c(uint64_t virtio_base, uint16_t device_id);
