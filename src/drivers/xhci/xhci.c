#include "xhci.h"

typedef struct xhci_cap_registers {
    char caplength;
    char rsvd;
    uint16_t hci_version;
    uint32_t hcs_params_1;
    uint32_t hcs_params_2;
    uint32_t hcs_params_3;
    uint32_t hcc_params_1;
    uint32_t dboff;
    uint32_t rtsoff;
    uint32_t hcc_params_2;
} xhci_cap_registers; __attribute__((packed));

typedef struct xhci_op_registers {
    uint32_t usbcmd;
    uint32_t usbsts;
    uint64_t pagesize;
    uint32_t padding;
    uint32_t dnctrl;
    uint64_t crcr;
    uint32_t padding2[4];
    /** device context base address array pointer */
    uint64_t dcbaap;
    uint64_t config;
} xhci_op_registers; __attribute__((packed));

typedef struct xhci_port_registers {
    uint32_t portsc;
    uint32_t portpmsc;
    uint32_t portli;
    uint32_t reserved;
} xhci_port_registers; __attribute__((packed));

typedef struct xhci_interrupter {
    uint32_t iman;
    uint32_t imod;
    uint16_t erstsz;
    uint64_t erstba;
    uint64_t erdp;
} xhci_interrupter; __attribute__((packed));

typedef struct xhci_runtime_registers {
    uint32_t mfindex;
    uint32_t padding[7];
    xhci_interrupter interrupter;
} xhci_runtime_registers; __attribute__((packed));

typedef struct xhci_slot_context {
    uint32_t registers[4];
    uint32_t rsvdo[4];
} xhci_slot_context; __attribute__((packed));

typedef struct xhci_endpoint_context {
    uint32_t registers[5];
    uint32_t rsvdo[3];
} xhci_endpoint_context; __attribute__((packed));

typedef struct xhci_device_context {
    xhci_slot_context slot;
    xhci_endpoint_context endpoints[31];
} xhci_device_context; __attribute__((packed));

typedef struct xhci_dcbaa {
    xhci_device_context* context[256];
} xhci_dcbaa; __attribute__((packed, aligned(64)));

typedef struct xhci_trb_generic {
    uint64_t dw01;
    uint32_t dw2;
    uint32_t dw3;
} xhci_trb_generic; __attribute__((packed, aligned(64)));

typedef struct xhci_command_ring {
    xhci_trb_generic* trb[256];
} xhci_command_ring; __attribute__((packed, aligned(64)));

typedef struct xhci_event_ring {
    xhci_trb_generic* trb[256];
} xhci_event_ring; __attribute__((packed, aligned(64)));

typedef struct xhci_erst_entry {
    uint64_t pa;
    uint32_t size;
    uint32_t reserved;  
} xhci_erst_entry; __attribute__((packed, aligned(64)));

/**
 * Check for new connections in enabled ports.
 * Supports up to 64 ports.
 * Returns a 1 in the corresponding bit for each enabled port number 
 * (e.g. Port 4 -> bit 3)
 */
uint64_t xhci_poll_ports(uint64_t* op_base) {
    uint64_t new_conn;
    for (int port_num = 0; port_num < 64; port_num++) {
        xhci_port_registers* port = (xhci_port_registers*)(op_base + 0x400 + (port_num)*0x10);
        if (port->portsc & 3 && (port->portsc >> 17) & 1) {
            new_conn |= 1 << port_num;
        }
    }
    return new_conn;
}

int c_init_xhci() {
    uint64_t xhci_base = find_pci_device(0x1b36, 0x000d);
    
    uint64_t pci_cmd = xhci_base + 0x04;
    uint64_t xhci_bar0 = xhci_base + 0x10;
    uint64_t xhci_bar1 = xhci_base + 0x14;

    
    mmio_write32(xhci_bar0, 0xffffffff);
    uint32_t size0 = ~(mmio_read32(xhci_bar0) & ~0xf) + 1;
    
    mmio_write32(xhci_bar1, 0xffffffff);
    uint32_t size1 = ~(mmio_read32(xhci_bar1) & ~0xf) + 1;
    
    c_dgb_hex(size0);
    c_dgb_hex(size1);
    
    // Allocate 16kb, 16kb-aligned for BAR0 and 16b, 16b-aligned for BAR1
    uint8_t* bar0_ptr = kmalloc_aligned(size0, size0);
    uint8_t* bar1_ptr = kmalloc_aligned(size1, size1); // additional registers
    
    c_dgb_hex((uint64_t)bar0_ptr);
    
    mmio_write32(xhci_bar0, (uint32_t)bar0_ptr);
    mmio_write32(xhci_bar1, (uint32_t)bar1_ptr);
    
    bool x = pci_enable_device_c(xhci_base);
    if (!x) return -1;
    
    xhci_cap_registers* cap = (xhci_cap_registers*)bar0_ptr;
    xhci_op_registers* op = (xhci_op_registers*)bar0_ptr + sizeof(xhci_cap_registers);
    
    while (((op->usbsts) >> 11) & 1) {}
    c_dbg("CNR0");
    
    uint64_t max_slots = (cap->hcs_params_1) & 0xff;
    c_dgb_hex(max_slots);
    if (!max_slots) return -2;
    op->config |= max_slots;
    
    xhci_dcbaa* dcbaap = (xhci_dcbaa*)kmalloc_aligned(sizeof(xhci_dcbaa), 64);
    op->dcbaap = (uint64_t)dcbaap;
    
    xhci_command_ring cmd = {0};
    for (int i = 0; i < 256; i++) {
        cmd.trb[i] = (xhci_trb_generic*)kmalloc_aligned(sizeof(xhci_trb_generic), 64);
        cmd.trb[i]->dw01 = 0;
        cmd.trb[i]->dw2 = 0;
        cmd.trb[i]->dw3 = 0;
    }
    cmd.trb[255]->dw01 = (uint64_t)cmd.trb[0];
    cmd.trb[255]->dw3 |= 0b11 << 11 | 0b11; 
    
    
    xhci_erst_entry* erst = (xhci_erst_entry*)kmalloc_aligned(sizeof(xhci_erst_entry),64);
    
    xhci_event_ring er = {0};
    for (int i = 0; i < 256; i++) {
        er.trb[i] = (xhci_trb_generic*)kmalloc_aligned(sizeof(xhci_trb_generic), 64);
        er.trb[i]->dw01 = 0;
        er.trb[i]->dw2 = 0;
        er.trb[i]->dw3 = 0;
    }
    
    erst->pa = (uint64_t)&er;
    erst->size = 256;
    erst->reserved = 0;

    xhci_runtime_registers* runtime = (xhci_runtime_registers*)bar0_ptr + ((cap->rtsoff & 0xfffffff0) >> 4);
    runtime->interrupter.erstsz=1;
    runtime->interrupter.erstba = (uint64_t)&erst;
    runtime->interrupter.erdp = (uint64_t)&er | 0b000;
    runtime->interrupter.iman |= 0b11;
    runtime->interrupter.imod = 0;  

    op->usbcmd |= 1;
    while (op->usbsts & 1) {}
    uint64_t* pci_cmd_ptr = (uint64_t*)pci_cmd;
    *pci_cmd_ptr &= ~0x200;

    uint32_t max_ports = (cap->hcs_params_1 >> 24) & 0xff;  
    uint64_t active_ports = xhci_poll_ports((uint64_t*)op);
    
    
    return 0;
}

