#![no_std]

#[cfg(feature = "vfs")]
use core::ptr::null_mut;

#[cfg(feature = "vfs")]
use vfs::FileOps;

const EBADF: isize = -9;
const ESPIPE: isize = -29;
const ENOTTY: isize = -25;

fn urandom_read(_file: *mut u8, buf: *mut u8, count: usize) -> isize {
    foundation::kfn::krandom(buf, count)
}

fn urandom_write(_file: *mut u8, _buf: *const u8, _count: usize) -> isize {
    EBADF
}

fn urandom_close(_file: *mut u8) -> isize {
    0 // No cleanup needed
}

fn urandom_seek(_file: *mut u8, _offset: isize, _whence: i32) -> isize {
    ESPIPE
}

fn urandom_ioctl(_file: *mut u8, _request: usize, _arg: usize) -> isize {
    ENOTTY
}

#[cfg(feature = "vfs")]
pub const URANDOM_FOPS: FileOps = FileOps {
    read: urandom_read,
    write: urandom_write,
    release: urandom_close,
    llseek: urandom_seek,
    ioctl: urandom_ioctl,
};

#[cfg(feature = "vfs")]
pub fn urandom_factory() -> vfs::FdEntry {
    vfs::FdEntry {
        ops: &URANDOM_FOPS,
        private_data: null_mut(), // No per-fd context
    }
}
