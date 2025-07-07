//! Paging implementation for the MVOS kernel
//! MVOS uses 3 levels of page tables, containing
//! 511 entries (L0), 8192 entries (L1, L2)
//! with 4KiB granule, for a total address space size of
//! 128TiB 

use core::arch::asm;

// Set up translation tables

/// Entry type of a L0 page table: Invalid or Pointer (to next level)
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
enum L0PageTableEntryType {
    Invalid,
    Pointer(*mut L1PageTable),
}

impl L0PageTableEntryType {
    /// Returns the contained value
    /// 
    /// This function panics if used on an `Invalid` value. Thus, it is only intended to be used in prototyping and debugging situations.
    fn unwrap(&self) -> *mut L1PageTable {
        match self {
            L0PageTableEntryType::Invalid => { panic!("[  PAGING   ] \x1b[1;31mERROR: used `unwrap` on an `L0PageTableEntryType::Invalid` value\x1b[0m")},
            L0PageTableEntryType::Pointer(p) => *p
        }
    }
}

/// Entry type of a L1 page table: Invalid or Pointer (to next level)
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
enum L1PageTableEntryType {
    Invalid,
    Pointer(*mut L2PageTable),
}

impl L1PageTableEntryType {
    /// Returns the contained value
    /// 
    /// This function panics if used on an `Invalid` value. Thus, it is only intended to be used in prototyping and debugging situations.
    fn unwrap(&self) -> *mut L2PageTable {
        match self {
            L1PageTableEntryType::Invalid => { panic!("[  PAGING   ] \x1b[1;31mERROR: used `unwrap` on an `L1PageTableEntryType::Invalid` value\x1b[0m")},
            L1PageTableEntryType::Pointer(p) => *p
        }
    }
}

/// Entry type of a L2 page table: Invalid or Block
#[repr(C, align(4096))]
#[derive(Clone, Copy)]
enum L2PageTableEntryType {
    Invalid,
    Block(u64),
}

impl L2PageTableEntryType {
    /// Returns the contained value
    /// 
    /// This function panics if used on an `Invalid` value. Thus, it is only intended to be used in prototyping and debugging situations.
    fn unwrap(&self) -> u64 {
        match self {
            L2PageTableEntryType::Invalid => { panic!("[  PAGING   ] \x1b[1;31mERROR: used `unwrap` on an `L2PageTableEntryType::Invalid` value\x1b[0m")},
            L2PageTableEntryType::Block(p) => *p
        }
    }
}

/// Level 0 Page table data structure
#[repr(C, align(4096))]
pub struct L0PageTable {
    entries: [L0PageTableEntryType; 512],
}

impl L0PageTable {
    /// Create a new L0 page table with `Invalid` entries
    fn new() -> Self {
        Self {
            entries: [L0PageTableEntryType::Invalid; 512],
        }
    }

    /// Insert an entry into a specific index of an L1 page table
    /// 
    /// This function will return an error if the passed `index` is over 511 (max index for L1 page table)
    fn add_entry(&mut self, ptr: L0PageTableEntryType, index: usize) -> Result<(), &'static str> {
        if index > 511 { return Err("Invalid index for level 0 page table address insertion"); }

        self.entries[index] = ptr;

        Ok(())
    }

    /// Returns the entry at a specific index
    fn read_index(&self, index: usize) -> L0PageTableEntryType {
        self.entries[index]
    }
}

/// Level 1 Page table data structure
#[repr(C, align(4096))]
struct L1PageTable {
    entries: [L1PageTableEntryType; 8192],
}

impl L1PageTable {
    /// Create a new L1 page table with `Invalid` entries
    fn new() -> Self {
        Self {
            entries: [L1PageTableEntryType::Invalid; 8192],
        }
    }

    /// Insert an entry into a specific index of an L1 page table
    /// 
    /// This function will return an error if the passed `index` is over 8191 (max index for L1 page table)
    fn add_entry(&mut self, ptr: L1PageTableEntryType, index: usize) -> Result<(), &'static str> {
        if index > 8191 { return Err("Invalid index for level 1 page table address insertion"); }

        self.entries[index] = ptr;

        Ok(())
    }

    /// Returns the entry at a specific index
    fn read_index(&self, index: usize) -> L1PageTableEntryType {
        self.entries[index]
    }
}

/// Level 2 Page table data structure
#[repr(C, align(4096))]
struct L2PageTable {
    entries: [L2PageTableEntryType; 8192],
}

impl L2PageTable {
    /// Create a new L2 page table with `Invalid` entries
    fn new() -> Self {
        Self {
            entries: [L2PageTableEntryType::Invalid; 8192],
        }
    }

    /// Insert an entry into a specific index of an L2 page table
    /// 
    /// Parameters: 
    ///     addr: Physical address of the block (Bits [47:16])
    ///     share: shareability attributes (Bits [9:8])
    ///     access: Access permission bits (Bits [7:6])
    ///     mair_index: index in the MAIR register (Bits [4:2])
    ///     entry_type: The type of the entry (Bits [1:0])
    /// 
    /// This function will return an error if the passed `index` is over 8191 (max index for L1 page table)
    fn add_entry(&mut self, addr: u64, share: u64, access: u64, mair_index: u64, entry_type: u64, index: usize) -> Result<(), &'static str> {
        if index > 8191 { return Err("Invalid index for level 2 page table address insertion"); }

        self.entries[index] = L2PageTableEntryType::Block(
            addr << 16 | share << 8 | access << 6 | mair_index << 2 | entry_type
        );

        Ok(())
    }

    /// Returns the entry at a specific index
    fn read_index(&self, index: usize) -> L2PageTableEntryType {
        self.entries[index]
    }
}

/// Traverse the page tables and translate a virtual address into a physical one.
/// 
/// This happens as such:
/// Bits [51:42] are used as an index into the L0 page table,
/// Bits [41:29] are used as an index into the L1 page table,
/// Bits [28:16] are used as an index into the L2 page table,
/// Bits [15:00] are used as an offset inside the page.
/// 
/// This function is unsafe because it needs to dereference raw pointers
pub unsafe fn translate_address(virt: u64, l0_table: &L0PageTable) -> Result<u64, &'static str> {
    let l0_index = ((virt >> 42) & ((1_u64 << 10) - 1)) as usize;
    let l1_index = ((virt >> 29) & ((1_u64 << 13) -1)) as usize;
    let l2_index = ((virt >> 16) & ((1_u64 << 13) -1)) as usize;
    let offset = virt & ((1_u64 << 16) -1);

    if let L0PageTableEntryType::Pointer(l1_table) = l0_table.read_index(l0_index) {
        if let L1PageTableEntryType::Pointer(l2_table) = (*l1_table).read_index(l1_index) { //FIXME: make this function memory safe somehow?
            if let L2PageTableEntryType::Block(block) = (*l2_table).read_index(l2_index) {
                return Ok(block + offset);
            } else {
                return Err("L2 table entry was `Invalid`");
            }
        } else {
            return Err("L1 table entry was `Invalid`");
        }
    } else {
        return Err("L0 table entry was `Invalid`");
    }
}

pub unsafe fn paging_idmap_setup() {
    let mut idmap = L0PageTable::new();
    let mut l1 = L1PageTable::new();
    let mut l2 = L2PageTable::new();
    l1.add_entry(L1PageTableEntryType::Pointer(&mut l2 as *mut L2PageTable), 0);
    idmap.add_entry(L0PageTableEntryType::Pointer(&mut l1 as *mut L1PageTable), 0);

    let block_size: u64 = 0x40000000000;
    let block_attr = 
        (1 << 0) |
        (2 << 2) |
        (1 << 6) |
        (3 << 8) |
        (1 << 10);
    let mut phys: u64 = 0;

    for i in 0..512 {
        l2.add_entry(phys, 1, 3, 1, 2, 0);
        phys += block_size;
    }

    asm!("msr ttbr1_el1, {}", in(reg) &mut idmap as *mut L0PageTable as u64);
}