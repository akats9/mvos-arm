#include <global_include.h>
#include "virtio_generic.h"
#include "../memory/mmu.h"

typedef uint32_t le32;
typedef uint16_t le16;
typedef uint64_t le64;

/* Virtio Capabilities structs. Here, only generic ones are defined. 
   Device specific caps are to be defined in the specific device driver.
   (e.g virtio_pci_cap_device_cfg)
*/

struct virtio_pci_cap { 
        u8 cap_vndr;    /* Generic PCI field: PCI_CAP_ID_VNDR */ 
        u8 cap_next;    /* Generic PCI field: next ptr. */ 
        u8 cap_len;     /* Generic PCI field: capability length */ 
        u8 cfg_type;    /* Identifies the structure. */ 
        u8 bar;         /* Where to find it. */ 
        u8 id;          /* Multiple capabilities of the same type */ 
        u8 padding[2];  /* Pad to full dword. */ 
        le32 offset;    /* Offset within bar. */ 
        le32 length;    /* Length of the structure, in bytes. */ 
}; __attribute__((packed));

// type = 1
typedef struct virtio_pci_common_cfg { 
        /* About the whole device. */ 
        le32 device_feature_select;     /* read-write */ 
        le32 device_feature;            /* read-only for driver */ 
        le32 driver_feature_select;     /* read-write */ 
        le32 driver_feature;            /* read-write */ 
        le16 config_msix_vector;        /* read-write */ 
        le16 num_queues;                /* read-only for driver */ 
        u8 device_status;               /* read-write */ 
        u8 config_generation;           /* read-only for driver */ 
 
        /* About a specific virtqueue. */ 
        le16 queue_select;              /* read-write */ 
        le16 queue_size;                /* read-write */ 
        le16 queue_msix_vector;         /* read-write */ 
        le16 queue_enable;              /* read-write */ 
        le16 queue_notify_off;          /* read-only for driver */ 
        le64 queue_desc;                /* read-write */ 
        le64 queue_driver;              /* read-write */ 
        le64 queue_device;              /* read-write */ 
        le16 queue_notif_config_data;   /* read-only for driver */ 
        le16 queue_reset;               /* read-write */ 
 
        /* About the administration virtqueue. */ 
        le16 admin_queue_index;         /* read-only for driver */ 
        le16 admin_queue_num;         /* read-only for driver */ 
} virtio_pci_common_cfg; __attribute__((packed));

// type = 2
typedef struct virtio_pci_notify_cap { 
        struct virtio_pci_cap cap; 
        le32 notify_off_multiplier; /* Multiplier for queue_notify_off. */ 
} virtio_pci_notify_cap; __attribute__((packed));

// type = 3
typedef struct virtio_pci_cap_isr_cfg {
    uint8_t isr;
} virtio_pci_cap_isr_cfg; __attribute__((packed));

int virtio_generic_setup_c(uint64_t virtio_base, uint16_t device_id) {
    uint64_t virtio_bar0 = virtio_base + 0x10;
    
    mmio_write32(virtio_bar0, 0xffffffff);
    c_dgb_hex(mmio_read32(virtio_bar0));
    uint32_t size0 = ~(mmio_read32(virtio_bar0) & ~0xf) + 1;
    
    c_dgb_hex(size0);
    
    // Allocate 16kb, 16kb-aligned for BAR0 and 16b, 16b-aligned for BAR1
    uint8_t* bar0_ptr = kmalloc_aligned(size0, size0);
    
    c_dgb_hex((uint64_t)bar0_ptr);
    
    mmio_write32(virtio_bar0, (uint32_t)bar0_ptr);
    uint64_t virtio_mmio_base = (uint64_t)bar0_ptr;

    uint64_t status_register = mmio_read64(virtio_mmio_base + 0x06);
    if (!((status_register >> 4) & 1)) return -1;
    uintptr_t capabilities_pointer = mmio_read64(virtio_mmio_base + 0x34);
    uint8_t common_cfg_bar;
    uint32_t common_cfg_offset, common_cfg_len;
    uint8_t notify_cfg_bar;
    uint32_t notify_cfg_offset, notify_cfg_len;
    uint8_t isr_cfg_bar;
    uint32_t isr_cfg_offset, isr_cfg_len;
    uint8_t device_cfg_bar;
    uint32_t device_cfg_offset, device_cfg_len;
    uint8_t timeout = 0;
    while (capabilities_pointer) {
        c_dbg("iteration");
        struct virtio_pci_cap* cap = (struct virtio_pci_cap*)(virtio_mmio_base + capabilities_pointer);
        if (cap->cap_vndr == 0x09) {
            switch (cap->cfg_type) {
                case VIRTIO_PCI_CAP_COMMON_CFG: {
                    common_cfg_bar = cap->bar;
                    common_cfg_len = cap->length;
                    common_cfg_offset = cap->offset;
                    c_dbg("found common cfg");
                }
                case VIRTIO_PCI_CAP_NOTIFY_CFG: {
                    notify_cfg_bar = cap->bar;
                    notify_cfg_offset = cap->offset;
                    notify_cfg_len = cap->length;
                    c_dbg("found notify cfg");
                }
                case VIRTIO_PCI_CAP_ISR_CFG: {
                    isr_cfg_bar = cap->bar;
                    isr_cfg_offset = cap->offset;
                    isr_cfg_len = cap->length;
                    c_dbg("found isr cfg");
                }
                case VIRTIO_PCI_CAP_DEVICE_CFG: {
                    device_cfg_bar = cap->bar;
                    device_cfg_offset = cap->offset;
                    device_cfg_len = cap->length;
                    c_dbg("found device cfg");
                }
                default: {};
            }
        }
        capabilities_pointer = cap->cap_next;
    }
    if (common_cfg_bar == 0) return -2;

    for (uint64_t addr = common_cfg_bar - 0x1000; addr <= common_cfg_bar + 0x1000; addr += GRANULE_4KB) mmu_map_4kb(addr, addr, MAIR_IDX_DEVICE, 1);
    virtio_pci_common_cfg* common_cfg = (virtio_pci_common_cfg*)(common_cfg_bar + common_cfg_offset);

    // Reset the device
    common_cfg->device_status = RESET;
    // Acknowledge the device
    common_cfg->device_status = DEVICE_ACK;
    // Tell the device we know how to drive it
    common_cfg->device_status = DRIVER_LOADED;

    // Read feature bits
    uint32_t device_features = common_cfg->device_feature;
    // Parse and negotiate features (for now, accept everything and pray to God)
    common_cfg->device_feature_select = device_features;

    // Set and verify FEATURES_OK
    common_cfg->device_status = FEATURES_OK;
    if (common_cfg->device_status != FEATURES_OK) common_cfg->device_status = DRIVER_FAILED; return -3;

    // Perform device specific setup based on device id
    switch (device_id) {
        // GPU
        case 0x10: {}
        default: {}        
    }

    return 0;
}