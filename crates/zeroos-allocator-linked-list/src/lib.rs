#![no_std]

mod allocator;

use foundation::ops::MemoryOps;

pub const LINKED_LIST_ALLOCATOR_OPS: MemoryOps = MemoryOps {
    init: allocator::init,
    alloc: allocator::alloc,
    dealloc: allocator::dealloc,
    realloc: allocator::realloc,
};
