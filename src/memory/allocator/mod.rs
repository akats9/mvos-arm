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

pub mod free_list;