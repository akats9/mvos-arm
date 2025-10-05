#include "ramfb.h"

typedef struct FWCfgFile {
    uint32_t size;
    uint16_t select;
    uint16_t reserved;
    uint8_t name[56];
} FWCfgFile;

struct __attribute__((packed)) FWCfgDmaAccess {
    uint32_t control;
    uint32_t len;
    uint64_t addr;
};

typedef struct __attribute__((packed)) RamFBCfg {
    uint64_t addr;
    uint32_t fmt;
    uint32_t flags;
    uint32_t w;
    uint32_t h;
    uint32_t st;
} RamFBCfg; 

void qemu_dma_transfer(u32 control, u32 len, u64 addr) {
    volatile u64* fw_cfg_dma = (volatile u64*)0x9020010;
    
    static volatile struct FWCfgDmaAccess dma __attribute__((aligned(8)));
    dma.control = __builtin_bswap32(control);
    dma.len = __builtin_bswap32(len);
    dma.addr = __builtin_bswap64(addr);
    
    u64 dma_addr = (u64)&dma;

    *fw_cfg_dma = __builtin_bswap64(dma_addr);

    asm volatile ("dmb sy" ::: "memory");

    c_dbg("Enter loop ☦️");

    while (dma.control & ~__builtin_bswap32(QEMU_CFG_DMA_CTL_ERROR)) asm volatile ("" ::: "memory");
    if ((__builtin_bswap32(dma.control) & QEMU_CFG_DMA_CTL_ERROR) == 1) c_serial_println("[   RAMFB   ] \x1b[0;31mAn error occured in qemu_dma_transfer\x1b[0m");
}

boolean compare_etc_ramfb(uint8_t* bytes, size_t len) {
    const uint8_t ramfb_key[] = "etc/ramfb";
    size_t null_pos = 0;

    // Find null terminator
    for (size_t i = 0; i < len; i++) {
        if (bytes[i] == 0) {
            null_pos = i;
            break;
        }
    }

    // If no null terminator found, print warning and assume null_pos = 0
    if (null_pos == 0 && len > 0 && bytes[0] != 0) {
        c_serial_println("[   RAMFB   ] WARNING: compare_etc_ramfb(): input buffer is not null terminated.");
    }

    // Compare bytes up to null_pos with ramfb_key
    size_t key_len = sizeof(ramfb_key) - 1; // Exclude null terminator
    if (null_pos != key_len) {
        return false; // Length mismatch
    }

    for (size_t i = 0; i < null_pos; i++) {
        if (bytes[i] != ramfb_key[i]) {
            return false; // Byte mismatch
        }
    }

    return true; // Match
}

void c_setup_ramfb(char* fb_addr, u32 width, u32 height) {
    u32 num_entries = 0xffffffff;
    u32 fw_cfg_file_directory = 0x19;
    qemu_dma_transfer((u32)(fw_cfg_file_directory << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_READ), sizeof(u32), (u64)&num_entries);
    num_entries = __builtin_bswap32(num_entries);
    struct FWCfgFile ramfb = {0};

    for (u32 i = 0; i < num_entries; i++) {
        qemu_dma_transfer(QEMU_CFG_DMA_CTL_READ, (u32)sizeof(FWCfgFile), (u64)&ramfb);

        if (compare_etc_ramfb(ramfb.name, 56)) { c_serial_println("[   RAMFB   ] \x1b[0;32mFound entry \"etc/ramfb\", break.\x1b[0;m"); break; }
        else c_serial_println("[   RAMFB   ] \x1b[0;31mNo entry found\x1b[0m");
    }

    //u32 pixel_format = ((u32)'R') | (((u32)'X') << 8) | (((u32)'2') << 16) | (((u32)'4') << 24);
    u32 fourcc = 0x34325258;
    u32 bpp = BPP;

    struct RamFBCfg ramfb_cfg = {
        __builtin_bswap64((u64)fb_addr),
        __builtin_bswap32(fourcc),
        0,
        __builtin_bswap32((u32)width),
        __builtin_bswap32((u32)height),
        __builtin_bswap32((u32)width * bpp),
    };

    c_dbg("RamFB config:");
    c_dbg("  addr: ");
    c_dgb_hex(__builtin_bswap64(ramfb_cfg.addr));
    c_dbg("  fourcc: ");
    c_dgb_hex(__builtin_bswap32(ramfb_cfg.fmt));
    c_dbg("  flags: ");
    c_dgb_hex(__builtin_bswap32(ramfb_cfg.flags));
    c_dbg("  width: ");
    c_dgb_hex(__builtin_bswap32(ramfb_cfg.w));
    c_dbg("  height: ");
    c_dgb_hex(__builtin_bswap32(ramfb_cfg.h));
    c_dbg("  stride: ");
    c_dgb_hex(__builtin_bswap32(ramfb_cfg.st));

    c_dbg("RamFB select:");
    c_dgb_hex(ramfb.select);

    c_dbg("Full control word:");
    c_dgb_hex(((u32)(ramfb.select)) << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_WRITE);

    fb_addr[0] = 0xde;
    asm volatile ("dc cvac, %0" :: "r"(&fb_addr[0]) : "memory");
    char readback = fb_addr[0];
    if (readback == 0xde) c_dbg("FB works");
    else c_dbg("FB does not work");

    qemu_dma_transfer(((u32)__builtin_bswap16(ramfb.select)) << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_WRITE, (u32)sizeof(RamFBCfg), (u64)&ramfb_cfg);
}

void ramfb_clear(u8 color, char* fb_addr) {
    for (u32 x = 0; x < BPP*SCREENWIDTH*SCREENHEIGHT; x++) {
        *(fb_addr + x) = color;
    }
}