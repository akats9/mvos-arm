use alloc::alloc::{Layout, GlobalAlloc};
use core::{
    ptr::{
        self, NonNull},
        mem,
};

use crate::memory::allocator::fixed_size_block::FixedSizeBlockAllocator;

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

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

///Align the given address upwards to given alignment
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr //already aligned
    } else { 
        addr - remainder + align
    }
}

pub mod fixed_size_block;
pub mod linked_list;