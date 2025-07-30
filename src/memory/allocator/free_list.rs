//! MVOS Free list memory allocator implementation

use core::{alloc::GlobalAlloc, ptr::addr_of_mut};

use crate::memory::allocator::align_up;

unsafe extern "C" {
    pub unsafe static heap_bottom: u8;
}

#[repr(C)]
pub struct FreeBlock {
    addr: *mut u8,
    size: usize,
    next: Option<*mut FreeBlock>,
}

unsafe impl Send for FreeBlock {}
unsafe impl Sync for FreeBlock {}

impl FreeBlock {
    /// Default head of list starting at a dummy address with 4KiB size.
    ///
    /// The list will be initialized properly at runtime.
    pub const unsafe fn new_list() -> Self {
        Self {
            addr: 0x41000000 as *mut u8,
            size: 4096,
            next: None,
        }
    }

    /// New instance of block from provided size and address to push onto the list.
    pub fn new(addr: *mut u8, size: usize) -> Self {
        Self {
            addr, size, next: None,
        }
    }
}

#[repr(C)]
pub struct FreeListAllocator {
    head: *mut FreeBlock,
}

impl FreeListAllocator {
    pub const fn new() -> Self {
        unsafe {
            let mut head = FreeBlock::new_list();
            Self {
                head: &raw mut head,
            }
        }
    }
}

unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        let rounded = align_up(size, align);

        // Traverse the free list and pop the final block.

        let mut current_block = self.head;
        let mut prev_block: *mut FreeBlock = 0 as *mut FreeBlock;

        while (*current_block).next.is_some() {
            if (*(*current_block).next.unwrap()).size >= rounded {
                prev_block = current_block;
                current_block = (*current_block).next.unwrap();
            } else {
                (*prev_block).next = (*current_block).next;
                return (*current_block).addr;
            }
        }

        // Return null pointer to indicate allocation failure

        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let size = align_up(layout.size(), layout.align());

        let mut block = FreeBlock::new(ptr, size);

        // Traverse the list and push new block. 
        let mut current_block = self.head;
        while (*current_block).next.is_some() {
            current_block = (*current_block).next.unwrap();
        }

        (*current_block).next = Some(addr_of_mut!(block));
    }
}

unsafe impl Send for FreeListAllocator {}
unsafe impl Sync for FreeListAllocator {}