use crate::kfn;

pub fn sys_exit(status: usize) -> isize {
    kfn::kexit(status as i32)
}

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

pub fn sys_clock_gettime(_clockid: usize, timespec: usize) -> isize {
    if timespec != 0 {
        unsafe {
            let ts = timespec as *mut Timespec;
            (*ts).tv_sec = 0;
            (*ts).tv_nsec = 0;
        }
    }
    0
}

#[repr(C)]
struct Rlimit64 {
    rlim_cur: u64,
    rlim_max: u64,
}

pub fn sys_prlimit64(_pid: usize, res: usize, _new_limit: usize, old_limit: usize) -> isize {
    if old_limit != 0 {
        let rlim = old_limit as *mut Rlimit64;
        unsafe {
            match res as i32 {
                3 => {
                    (*rlim).rlim_cur = 8 * 1024 * 1024;
                    (*rlim).rlim_max = 8 * 1024 * 1024;
                } // RLIMIT_STACK
                7 => {
                    (*rlim).rlim_cur = 1024;
                    (*rlim).rlim_max = 1024;
                } // RLIMIT_NOFILE
                9 => {
                    (*rlim).rlim_cur = 1024 * 1024 * 1024;
                    (*rlim).rlim_max = 1024 * 1024 * 1024;
                } // RLIMIT_AS
                _ => {
                    (*rlim).rlim_cur = u64::MAX;
                    (*rlim).rlim_max = u64::MAX;
                }
            }
        }
    }
    0
}

pub fn sys_rt_sigaction(_sig: usize, _act: usize, _oldact: usize) -> isize {
    0
}
pub fn sys_rt_sigprocmask(_how: usize, _set: usize, _oldset: usize) -> isize {
    0
}
