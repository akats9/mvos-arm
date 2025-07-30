#include "../include/types.h"
#include "pci.h"
#include "../include/mvos_bindings.h"

#define PCI_BAR_BASE_OFFSET 0x10

bool pci_enable_device_c(size_t base) {
    size_t* cmd_addr = (size_t*)(base + 0x04);
    size_t cmd_before = *cmd_addr;
    size_t cmd = cmd_before | 0x07;
    *cmd_addr = cmd;
    size_t cmd_after = *cmd_addr;
    if ((cmd_after & 0x7) == 0x7) {
        return true;
    } else {
        return false;
    }
}

