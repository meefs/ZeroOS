use core::alloc::{GlobalAlloc, Layout};

pub struct System;

unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        foundation::kfn::memory::kmalloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        foundation::kfn::memory::kfree(ptr, layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        foundation::kfn::memory::krealloc(ptr, layout, new_size)
    }
}
