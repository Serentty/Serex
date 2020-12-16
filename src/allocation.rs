use alloc::alloc::Layout;

const HEAP_BASE: usize = 32 * 1024 * 1024; // The heap starts 32 MiB in for now.
const HEAP_SIZE: usize = 2 * 1024 * 1024 * 1024; // 1 GiB

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation failed.\n{:?}", layout)
}

pub fn initialize() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_BASE, HEAP_SIZE);
    }
}