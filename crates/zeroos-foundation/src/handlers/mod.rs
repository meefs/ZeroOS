pub mod info;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "random")]
pub mod random;
#[cfg(feature = "scheduler")]
pub mod thread;
#[cfg(feature = "vfs")]
pub mod vfs;

#[derive(Debug, Clone, Copy)]
pub struct HandlerContext {
    pub mepc: usize,
    pub frame_ptr: usize,
}

impl HandlerContext {
    #[inline]
    pub const fn new(mepc: usize, frame_ptr: usize) -> Self {
        Self { mepc, frame_ptr }
    }
}

pub type SyscallHandler = fn(
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    ctx: &HandlerContext,
) -> isize;

pub fn sys_unsupported() -> isize {
    -errno::ENOSYS
}

pub fn sys_noop() -> isize {
    0
}

pub mod errno {
    pub const ENOSYS: isize = 38;
    pub const EBADF: isize = 9;
    pub const EINVAL: isize = 22;
    pub const ENOMEM: isize = 12;
    pub const EAGAIN: isize = 11;
    pub const ENOTTY: isize = 25;
    pub const ESPIPE: isize = 29;
    pub const ENOENT: isize = 2;
    pub const EPERM: isize = 1;
}
