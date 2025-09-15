//use alloc::alloc::{Layout, GlobalAlloc};
use core::{
    ptr::{
        self, NonNull},
        mem,
};
use linked_list_allocator::LockedHeap;

use crate::memory::allocator::free_list::{FreeBlock, FreeListAllocator};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    let heap_start = 0x41000000;
    let heap_end = 0x42000000;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
}

/// Wrapper for spin::Mutex to permit trait impl
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

#[unsafe(no_mangle)]
///Align the given address upwards to given alignment
pub extern "C" fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr //already aligned
    } else { 
        addr - remainder + align
    }
}

pub mod alloc_ffi {
    use core::{alloc::{Layout, GlobalAlloc}, ptr::{null, null_mut}};

    use crate::memory::allocator::ALLOCATOR;

    #[unsafe(no_mangle)]
    pub extern "C" fn kmalloc(size: usize) -> *mut u8 {
        if size == 0 {
            return core::ptr::null_mut();
        }

        let layout = match Layout::from_size_align(size, 1) {
            Ok(layout) => layout,
            Err(_) => return core::ptr::null_mut(),
        };

        unsafe {
            ALLOCATOR.alloc(layout)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn kmalloc_aligned(size: usize, align: usize) -> *mut u8 {
        if size == 0 {
            return null_mut();
        }

        let layout = match Layout::from_size_align(size, align) {
            Ok(layout) => layout,
            Err(_) => return null_mut(),
        };

        unsafe {
            ALLOCATOR.alloc(layout)
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn kfree(ptr: *mut u8, size: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }

        let layout = match Layout::from_size_align(size, 1) {
            Ok(layout) => layout,
            Err(_) => return,
        };

        unsafe {
            ALLOCATOR.dealloc(ptr, layout);
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn kfree_aligned(ptr: *mut u8, size: usize, align: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }

        let layout = match Layout::from_size_align(size, align) {
            Ok(layout) => layout,
            Err(_) => return,
        };

        unsafe {
            ALLOCATOR.dealloc(ptr, layout);
        }
    }
}

pub mod free_list;