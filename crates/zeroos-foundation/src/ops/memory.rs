use core::alloc::Layout;

#[derive(Clone, Copy)]
pub struct MemoryOps {
    pub init: fn(heap_start: usize, heap_size: usize),
    pub alloc: fn(layout: Layout) -> *mut u8,
    pub dealloc: fn(ptr: *mut u8, layout: Layout),
    pub realloc: fn(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8,
}
