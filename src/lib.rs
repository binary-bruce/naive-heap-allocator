#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use buddy_system_allocator::LockedHeap;

#[global_allocator]
/// heap allocator instance
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
/// panic when heap allocation error occurs
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

/// initiate heap allocator
pub fn init_heap(start: usize, size: usize) {
    unsafe {
        HEAP_ALLOCATOR.lock().init(start, size);
    }
}
