// https://grasslab.github.io/NYCU_Operating_System_Capstone/labs/lab8.html

#include "../include/types.h"
#include "../include/mvos_bindings.h"
#include "paging.h"

// TCR_EL1

#define TCR_CONFIG_REGION_48bit (((64-48) << 0) | ((64 - 48) << 16))
#define TCR_CONFIG_4KB ((0b00 << 14) | (0b10 << 30))
#define TCR_CONFIG_DEFAULT (TCR_CONFIG_REGION_48bit | TCR_CONFIG_4KB)

// MAIR_EL1

#define MAIR_DEVICE_nGnRnE 0b00000000
#define MAIR_NORMAL_NOCACHE 0b01000100
#define MAIR_IDX_DEVICE_nGnRnE 0
#define MAIR_IDX_NORMAL_NOCACHE 1

// Identity Paging

#define PD_TABLE 0b11
#define PD_BLOCK 0b01
#define PD_ACCESS (1 << 10)
#define BOOT_PGD_ATTR PD_TABLE
#define BOOT_PUD_ATTR (PD_ACCESS | (MAIR_IDX_DEVICE_nGnRnE << 2) | PD_BLOCK)

// void paging_boot() {
//     // Set up TCR_EL1.
//     load_config_tcr_el1(TCR_CONFIG_DEFAULT);
//     // Set up MAIR_EL1
//     size_t config = ((MAIR_DEVICE_nGnRnE << (MAIR_IDX_DEVICE_nGnRnE * 8)) | (MAIR_NORMAL_NOCACHE << (MAIR_IDX_NORMAL_NOCACHE * 8)));
//     load_config_mair_el1(config);
//     identity_paging_setup(BOOT_PGD_ATTR, BOOT_PUD_ATTR);
// }