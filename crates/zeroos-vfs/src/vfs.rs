use crate::*;

pub struct Vfs {
    fd_table: [Option<FdEntry>; MAX_FDS],
    next_fd: Fd,
    devices: [(Option<&'static str>, Option<DeviceFactory>); 32],
}

unsafe impl Send for Vfs {}
unsafe impl Sync for Vfs {}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

impl Vfs {
    pub const fn new() -> Self {
        const NONE: (Option<&'static str>, Option<DeviceFactory>) = (None, None);
        Self {
            fd_table: [None; MAX_FDS],
            next_fd: 3, // Start after stdio
            devices: [NONE; 32],
        }
    }

    pub fn register_fd(&mut self, fd: Fd, entry: FdEntry) -> VfsResult<()> {
        if fd < 0 || fd as usize >= MAX_FDS {
            return Err(EINVAL);
        }
        self.fd_table[fd as usize] = Some(entry);
        Ok(())
    }

    pub fn register_device(&mut self, path: &'static str, factory: DeviceFactory) -> VfsResult<()> {
        for entry in &mut self.devices {
            if entry.0.is_none() {
                *entry = (Some(path), Some(factory));
                return Ok(());
            }
        }
        Err(ENOMEM)
    }

    pub fn open(&mut self, path: &str, _flags: i32, _mode: u32) -> VfsResult<Fd> {
        let factory = self
            .devices
            .iter()
            .find(|(p, _)| p.is_some_and(|device_path| device_path == path))
            .and_then(|(_, f)| *f)
            .ok_or(ENOENT)?;

        if self.next_fd as usize >= MAX_FDS {
            return Err(EMFILE);
        }

        let fd = self.next_fd;
        self.next_fd += 1;

        let entry = factory();
        self.fd_table[fd as usize] = Some(entry);

        Ok(fd)
    }

    pub fn read(&self, fd: Fd, buf: *mut u8, count: usize) -> isize {
        if fd < 0 || fd as usize >= MAX_FDS {
            return EBADF;
        }

        match self.fd_table[fd as usize] {
            Some(entry) => (entry.ops.read)(entry.private_data, buf, count),
            None => EBADF,
        }
    }

    pub fn write(&self, fd: Fd, buf: *const u8, count: usize) -> isize {
        if fd < 0 || fd as usize >= MAX_FDS {
            return EBADF;
        }

        match self.fd_table[fd as usize] {
            Some(entry) => (entry.ops.write)(entry.private_data, buf, count),
            None => EBADF,
        }
    }

    pub fn lseek(&self, fd: Fd, offset: isize, whence: i32) -> isize {
        if fd < 0 || fd as usize >= MAX_FDS {
            return EBADF;
        }

        match self.fd_table[fd as usize] {
            Some(entry) => (entry.ops.llseek)(entry.private_data, offset, whence),
            None => EBADF,
        }
    }

    pub fn ioctl(&self, fd: Fd, request: usize, arg: usize) -> isize {
        if fd < 0 || fd as usize >= MAX_FDS {
            return EBADF;
        }

        match self.fd_table[fd as usize] {
            Some(entry) => (entry.ops.ioctl)(entry.private_data, request, arg),
            None => EBADF,
        }
    }

    pub fn close(&mut self, fd: Fd) -> isize {
        if fd < 0 || fd as usize >= MAX_FDS {
            return EBADF;
        }

        match self.fd_table[fd as usize].take() {
            Some(entry) => (entry.ops.release)(entry.private_data),
            None => EBADF,
        }
    }

    pub fn fstat(&self, fd: Fd, statbuf: *mut Stat) -> isize {
        if fd < 0 || fd as usize >= MAX_FDS {
            return EBADF;
        }

        if statbuf.is_null() {
            return EINVAL;
        }

        ENOSYS
    }
}

pub(crate) static mut VFS: Vfs = Vfs::new();

pub fn register_fd(fd: Fd, entry: FdEntry) -> VfsResult<()> {
    unsafe { VFS.register_fd(fd, entry) }
}

pub fn register_device(path: &'static str, factory: DeviceFactory) -> VfsResult<()> {
    unsafe { VFS.register_device(path, factory) }
}

// NOTE: These functions should NOT be called directly!

pub fn read(fd: Fd, buf: *mut u8, count: usize) -> isize {
    unsafe { VFS.read(fd, buf, count) }
}

pub fn write(fd: Fd, buf: *const u8, count: usize) -> isize {
    unsafe { VFS.write(fd, buf, count) }
}

pub fn lseek(fd: Fd, offset: isize, whence: i32) -> isize {
    unsafe { VFS.lseek(fd, offset, whence) }
}

pub fn ioctl(fd: Fd, request: usize, arg: usize) -> isize {
    unsafe { VFS.ioctl(fd, request, arg) }
}

pub fn close(fd: Fd) -> isize {
    unsafe { VFS.close(fd) }
}

pub fn fstat(fd: Fd, statbuf: *mut Stat) -> isize {
    unsafe { VFS.fstat(fd, statbuf) }
}

pub(crate) fn fstat_raw(fd: Fd, statbuf: *mut u8) -> isize {
    fstat(fd, statbuf as *mut Stat)
}

pub const VFS_OPS: crate::VfsOps = crate::VfsOps {
    read,
    write,
    open: open_cstr,
    close,
    lseek,
    ioctl,
    fstat: fstat_raw,
};

pub fn open_cstr(path: *const u8, flags: i32, mode: u32) -> isize {
    if path.is_null() {
        return EINVAL;
    }

    unsafe {
        let mut len = 0;
        while *path.add(len) != 0 {
            len += 1;
            if len > 4096 {
                return ENAMETOOLONG;
            }
        }
        let slice = core::slice::from_raw_parts(path, len);
        match core::str::from_utf8(slice) {
            Ok(s) => match VFS.open(s, flags, mode) {
                Ok(fd) => fd as isize,
                Err(e) => e,
            },
            Err(_) => EINVAL,
        }
    }
}
