use super::errno;
use crate::kfn;

pub fn sys_openat(_dirfd: usize, path: usize, flags: usize, mode: usize) -> isize {
    kfn::kopen(path as *const u8, flags as i32, mode as u32)
}

pub fn sys_close(fd: usize) -> isize {
    kfn::kclose(fd as i32)
}

pub fn sys_read(fd: usize, buf: usize, count: usize) -> isize {
    kfn::kread(fd as i32, buf as *mut u8, count)
}

pub fn sys_write(fd: usize, buf: usize, count: usize) -> isize {
    kfn::kwrite(fd as i32, buf as *const u8, count)
}

#[repr(C)]
struct IoVec {
    iov_base: *mut u8,
    iov_len: usize,
}

pub fn sys_readv(fd: usize, iov: usize, iovcnt: usize) -> isize {
    if iovcnt == 0 {
        return -errno::EINVAL;
    }
    let iovecs = unsafe { core::slice::from_raw_parts(iov as *const IoVec, iovcnt) };
    let mut total = 0isize;
    for v in iovecs {
        let r = kfn::kread(fd as i32, v.iov_base, v.iov_len);
        if r < 0 {
            return if total > 0 { total } else { r };
        }
        total += r;
        if (r as usize) < v.iov_len {
            break;
        }
    }
    total
}

pub fn sys_writev(fd: usize, iov: usize, iovcnt: usize) -> isize {
    if iovcnt == 0 {
        return -errno::EINVAL;
    }
    let iovecs = unsafe { core::slice::from_raw_parts(iov as *const IoVec, iovcnt) };
    let mut total = 0isize;
    for v in iovecs {
        let r = kfn::kwrite(fd as i32, v.iov_base as *const u8, v.iov_len);
        if r < 0 {
            return if total > 0 { total } else { r };
        }
        total += r;
        if (r as usize) < v.iov_len {
            break;
        }
    }
    total
}

pub fn sys_lseek(fd: usize, offset: usize, whence: usize) -> isize {
    kfn::klseek(fd as i32, offset as isize, whence as i32)
}

pub fn sys_ioctl(fd: usize, request: usize, arg: usize) -> isize {
    kfn::kioctl(fd as i32, request, arg)
}

pub fn sys_fstat(fd: usize, statbuf: usize) -> isize {
    kfn::kfstat(fd as i32, statbuf as *mut u8)
}
