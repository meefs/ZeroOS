use super::errno;
use core::alloc::Layout;

const PAGE_SIZE: usize = 4096;

pub fn sys_brk(_brk: usize) -> isize {
    -errno::ENOMEM
}

pub fn sys_mmap(
    addr: usize,
    len: usize,
    _prot: usize,
    _flags: usize,
    _fd: usize,
    _offset: usize,
) -> isize {
    let pages = len.div_ceil(PAGE_SIZE);
    let size = pages * PAGE_SIZE;
    let layout = match Layout::from_size_align(size, PAGE_SIZE) {
        Ok(l) => l,
        Err(_) => return -errno::EINVAL,
    };
    let ptr = crate::kfn::kmalloc(layout);
    if ptr.is_null() {
        return -errno::ENOMEM;
    }
    unsafe {
        core::ptr::write_bytes(ptr, 0, size);
    }
    let _ = addr;
    ptr as isize
}

pub fn sys_munmap(addr: usize, len: usize) -> isize {
    if addr == 0 || len == 0 {
        return -errno::EINVAL;
    }
    let pages = len.div_ceil(PAGE_SIZE);
    let size = pages * PAGE_SIZE;
    let layout = match Layout::from_size_align(size, PAGE_SIZE) {
        Ok(l) => l,
        Err(_) => return -errno::EINVAL,
    };
    crate::kfn::kfree(addr as *mut u8, layout);
    0
}

pub fn sys_mprotect(_addr: usize, _len: usize, _prot: usize) -> isize {
    0
}
