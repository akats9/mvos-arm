#include "../include/types.h"
#include "../include/mvos_bindings.h"
#include "paging.h"

uint32_t page_directory[1024] __attribute__((aligned(4096)));

/**
 * Call to init and blank the page directory.
 */
void blank_page_dir() {
    for (int i = 0; i < 1024; i++) {
        // This sets the following flags to the pages:
        //   Supervisor: Only kernel-mode can access them
        //   Write Enabled: It can be both read from and written to
        //   Not Present: The page table is not present
        page_directory[i] = 0x00000002;
    }
}

uint32_t first_page_table[1024] __attribute__((aligned(4096)));


void map_first_table() {
    // Holds the physical address where we want to start mapping these pages to.
    // In this case, we want to map these pages to the very beginning of memory.
    unsigned int i;

    // we will fill all 1024 entries in the table, mapping 4MiB
    for (i = 0; i < 1024; i++) {
        // As the address is page aligned, it will always leave 12 bits zeroed.
        // Those bits are used by the attributes
        first_page_table[i] = (i * 0x1000) | 3;
        // Attributes: supervisor level, r/w, present.
    }
}

void put_table_in_dir() {
    page_directory[0] = ((unsigned int)first_page_table) | 3;
}

void init_paging() {
    blank_page_dir();
    map_first_table();
    put_table_in_dir();
}