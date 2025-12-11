#![no_std]

pub use foundation::ops::VfsOps;

pub use libc::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH,
    S_IRUSR, S_IRWXG, S_IRWXO, S_IRWXU, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};

mod vfs;

pub use vfs::*;

pub type Fd = i32;

pub const STDIN_FD: Fd = 0;
pub const STDOUT_FD: Fd = 1;
pub const STDERR_FD: Fd = 2;

pub const MAX_FDS: usize = 256;

pub type VfsResult<T> = Result<T, isize>;

pub const EBADF: isize = -(libc::EBADF as isize); // Bad file descriptor
pub const ENOMEM: isize = -(libc::ENOMEM as isize); // Out of memory
pub const ENOENT: isize = -(libc::ENOENT as isize); // No such file or directory
pub const EINVAL: isize = -(libc::EINVAL as isize); // Invalid argument
pub const ESPIPE: isize = -(libc::ESPIPE as isize); // Illegal seek
pub const ENOTTY: isize = -(libc::ENOTTY as isize); // Not a typewriter (no ioctl)
pub const EMFILE: isize = -(libc::EMFILE as isize); // Too many open files
pub const ENAMETOOLONG: isize = -(libc::ENAMETOOLONG as isize); // File name too long
pub const ENOSYS: isize = -(libc::ENOSYS as isize); // Function not implemented

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Stat {
    pub st_dev: u64,     // Device ID
    pub st_ino: u64,     // Inode number
    pub st_mode: u32,    // File type and mode
    pub st_nlink: u32,   // Number of hard links
    pub st_uid: u32,     // User ID
    pub st_gid: u32,     // Group ID
    pub st_rdev: u64,    // Device ID (if special file)
    pub st_size: i64,    // File size in bytes
    pub st_blksize: i64, // Block size for I/O
    pub st_blocks: i64,  // Number of 512B blocks
    pub st_atime: i64,   // Access time (seconds since epoch)
    pub st_mtime: i64,   // Modification time
    pub st_ctime: i64,   // Change time
}

impl Stat {
    pub const fn zero() -> Self {
        Self {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atime: 0,
            st_mtime: 0,
            st_ctime: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FileOps {
    pub read: fn(file: *mut u8, buf: *mut u8, count: usize) -> isize,
    pub write: fn(file: *mut u8, buf: *const u8, count: usize) -> isize,
    pub release: fn(file: *mut u8) -> isize,
    pub llseek: fn(file: *mut u8, offset: isize, whence: i32) -> isize,
    pub ioctl: fn(file: *mut u8, request: usize, arg: usize) -> isize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FdEntry {
    pub ops: &'static FileOps,
    pub private_data: *mut u8,
}

pub type DeviceFactory = fn() -> FdEntry;

pub fn noop_close(_file: *mut u8) -> isize {
    0
}

pub fn noop_seek(_file: *mut u8, _offset: isize, _whence: i32) -> isize {
    ESPIPE
}

pub fn noop_ioctl(_file: *mut u8, _request: usize, _arg: usize) -> isize {
    ENOTTY
}

pub fn noop_read(_file: *mut u8, _buf: *mut u8, _count: usize) -> isize {
    EBADF
}

pub fn noop_write(_file: *mut u8, _buf: *const u8, _count: usize) -> isize {
    EBADF
}
