use core::alloc::Layout;
use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

const MIN_BLOCK_SIZE: usize = 64;

const MAX_ORDER: usize = 20;

const NUM_ORDERS: usize = MAX_ORDER + 1;

pub(crate) struct BuddyAllocator {
    heap_start: AtomicU64,
    heap_size: AtomicU64,
    free_lists: [AtomicU64; NUM_ORDERS],
}

impl BuddyAllocator {
    pub(crate) const fn new() -> Self {
        const ATOMIC_ZERO: AtomicU64 = AtomicU64::new(0);
        Self {
            heap_start: AtomicU64::new(0),
            heap_size: AtomicU64::new(0),
            free_lists: [ATOMIC_ZERO; NUM_ORDERS],
        }
    }

    pub(crate) fn init(&self, heap_start: usize, heap_size: usize) {
        let heap_size = heap_size.next_power_of_two() / 2;

        self.heap_start.store(heap_start as u64, Ordering::SeqCst);
        self.heap_size.store(heap_size as u64, Ordering::SeqCst);

        let max_order = order_for_size(heap_size);
        if max_order < NUM_ORDERS {
            self.free_lists[max_order].store(1, Ordering::SeqCst);
        }

        for i in 0..NUM_ORDERS {
            if i != max_order {
                self.free_lists[i].store(0, Ordering::SeqCst);
            }
        }
    }

    pub(crate) fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(MIN_BLOCK_SIZE).next_power_of_two();
        let order = order_for_size(size);

        if order >= NUM_ORDERS {
            return ptr::null_mut();
        }

        let block_index = match self.find_free_block(order) {
            Some(idx) => idx,
            None => return ptr::null_mut(),
        };

        let block_addr =
            self.heap_start.load(Ordering::Acquire) as usize + block_index * block_size(order);

        block_addr as *mut u8
    }

    pub(crate) fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }

        let size = layout.size().max(MIN_BLOCK_SIZE).next_power_of_two();
        let order = order_for_size(size);

        if order >= NUM_ORDERS {
            return;
        }

        let heap_start = self.heap_start.load(Ordering::Acquire) as usize;
        let offset = ptr as usize - heap_start;
        let block_index = offset / block_size(order);

        self.free_block(order, block_index);
    }

    fn find_free_block(&self, order: usize) -> Option<usize> {
        let mut free_mask = self.free_lists[order].load(Ordering::Acquire);
        if free_mask != 0 {
            let block_index = free_mask.trailing_zeros() as usize;
            free_mask &= !(1u64 << block_index);
            self.free_lists[order].store(free_mask, Ordering::Release);
            return Some(block_index);
        }

        for higher_order in (order + 1)..NUM_ORDERS {
            free_mask = self.free_lists[higher_order].load(Ordering::Acquire);
            if free_mask != 0 {
                return self.split_block(higher_order, order);
            }
        }

        None
    }

    fn split_block(&self, higher_order: usize, target_order: usize) -> Option<usize> {
        if higher_order <= target_order {
            return None;
        }

        let mut free_mask = self.free_lists[higher_order].load(Ordering::Acquire);
        if free_mask == 0 {
            return None;
        }

        let block_index = free_mask.trailing_zeros() as usize;
        free_mask &= !(1u64 << block_index);
        self.free_lists[higher_order].store(free_mask, Ordering::Release);

        let mut current_order = higher_order;
        let mut current_index = block_index;

        while current_order > target_order {
            current_order -= 1;
            let buddy_index = current_index * 2 + 1;
            current_index *= 2;

            let mut buddy_mask = self.free_lists[current_order].load(Ordering::Acquire);
            buddy_mask |= 1u64 << (buddy_index % 64);
            self.free_lists[current_order].store(buddy_mask, Ordering::Release);
        }

        Some(current_index)
    }

    fn free_block(&self, order: usize, block_index: usize) {
        if order >= MAX_ORDER {
            let mut free_mask = self.free_lists[order].load(Ordering::Acquire);
            free_mask |= 1u64 << (block_index % 64);
            self.free_lists[order].store(free_mask, Ordering::Release);
            return;
        }

        let buddy_index = block_index ^ 1;
        let mut free_mask = self.free_lists[order].load(Ordering::Acquire);

        if (free_mask & (1u64 << (buddy_index % 64))) != 0 {
            free_mask &= !(1u64 << (buddy_index % 64));
            free_mask &= !(1u64 << (block_index % 64));
            self.free_lists[order].store(free_mask, Ordering::Release);

            let merged_index = block_index / 2;
            self.free_block(order + 1, merged_index);
        } else {
            free_mask |= 1u64 << (block_index % 64);
            self.free_lists[order].store(free_mask, Ordering::Release);
        }
    }
}

#[inline]
fn order_for_size(size: usize) -> usize {
    if size <= MIN_BLOCK_SIZE {
        return 0;
    }
    let blocks = size.div_ceil(MIN_BLOCK_SIZE);
    (usize::BITS - blocks.leading_zeros() - 1) as usize
}

#[inline]
fn block_size(order: usize) -> usize {
    MIN_BLOCK_SIZE << order
}

pub(crate) static ALLOCATOR: BuddyAllocator = BuddyAllocator::new();

pub(crate) fn init(heap_start: usize, heap_size: usize) {
    ALLOCATOR.init(heap_start, heap_size);
}

pub(crate) fn alloc(layout: Layout) -> *mut u8 {
    ALLOCATOR.alloc(layout)
}

pub(crate) fn dealloc(ptr: *mut u8, layout: Layout) {
    ALLOCATOR.dealloc(ptr, layout);
}

pub(crate) fn realloc(ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
    if ptr.is_null() {
        let new_layout = match Layout::from_size_align(new_size, old_layout.align()) {
            Ok(l) => l,
            Err(_) => return ptr::null_mut(),
        };
        return alloc(new_layout);
    }

    if new_size == 0 {
        dealloc(ptr, old_layout);
        return ptr::null_mut();
    }

    let old_block_size = old_layout.size().max(MIN_BLOCK_SIZE).next_power_of_two();
    let new_block_size = new_size.max(MIN_BLOCK_SIZE).next_power_of_two();

    if old_block_size == new_block_size {
        return ptr;
    }

    let new_layout = match Layout::from_size_align(new_size, old_layout.align()) {
        Ok(l) => l,
        Err(_) => return ptr::null_mut(),
    };

    let new_ptr = alloc(new_layout);
    if !new_ptr.is_null() {
        let copy_size = old_layout.size().min(new_size);
        unsafe {
            ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
        }
        dealloc(ptr, old_layout);
    }
    new_ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_calculation() {
        assert_eq!(order_for_size(64), 0);
        assert_eq!(order_for_size(128), 1);
        assert_eq!(order_for_size(256), 2);
        assert_eq!(order_for_size(100), 1); // Rounds up to 128
    }

    #[test]
    fn test_block_size() {
        assert_eq!(block_size(0), 64);
        assert_eq!(block_size(1), 128);
        assert_eq!(block_size(2), 256);
    }

    #[test]
    fn test_buddy_alloc() {
        const HEAP_SIZE: usize = 1024 * 1024;
        let mut heap_mem = vec![0u8; HEAP_SIZE];
        let heap_start = heap_mem.as_mut_ptr() as usize;

        init(heap_start, HEAP_SIZE);

        let layout = Layout::from_size_align(128, 8).unwrap();
        let ptr = alloc(layout);
        assert!(!ptr.is_null());

        dealloc(ptr, layout);
    }
}
