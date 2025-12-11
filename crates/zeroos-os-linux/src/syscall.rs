#![allow(non_upper_case_globals)]

use cfg_if::cfg_if;
use foundation::handlers::{self, HandlerContext};
use libc::{self, c_long, *};

extern crate arch_riscv;
use arch_riscv::PtRegs;

macro_rules! define_call {
    (0, $name:ident) => {
        #[inline(always)]
        fn $name<F>(regs: *mut PtRegs, f: F)
        where
            F: FnOnce() -> isize,
        {
            let ret = f();
            unsafe {
                (*regs).a0 = ret as usize;
            }
        }
    };
    ($n:literal, $name:ident, $( $field:ident : $ty:ty ),+ ) => {
        #[inline(always)]
        fn $name<F>(regs: *mut PtRegs, f: F)
        where
            F: FnOnce($( $ty ),+) -> isize,
        {
            let r = unsafe { &*regs };
            let ret = f($( r.$field as $ty ),+);
            unsafe {
                (*regs).a0 = ret as usize;
            }
        }
    };
}

define_call!(0, call0);
define_call!(1, call1, a0: usize);
define_call!(2, call2, a0: usize, a1: usize);
define_call!(3, call3, a0: usize, a1: usize, a2: usize);
define_call!(4, call4, a0: usize, a1: usize, a2: usize, a3: usize);
define_call!(5, call5, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize);
define_call!(6, call6, a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize);

pub fn dispatch_syscall(regs: *mut PtRegs) {
    let regs_ref = unsafe { &*regs };

    let nr = regs_ref.a7;
    let mepc = regs_ref.mepc;
    let frame_ptr = regs as usize;

    let ctx = HandlerContext::new(mepc, frame_ptr);
    let nr = nr as c_long;

    // `foundation::kfn::update_frame` here to avoid double-updates or

    cfg_if! { if #[cfg(feature = "memory")] {
        match nr {
            SYS_brk => return call1(regs, handlers::memory::sys_brk),
            SYS_mmap => return call6(regs, |addr, len, prot, flags, fd, offset| {
                handlers::memory::sys_mmap(addr, len, prot, flags, fd, offset)
            }),
            SYS_munmap => return call2(regs, handlers::memory::sys_munmap),
            SYS_mprotect => return call3(regs, |addr, len, prot| {
                handlers::memory::sys_mprotect(addr, len, prot)
            }),
            SYS_madvise => return call3(regs, |_addr, _len, _advice| 0), // No-op: hints are advisory, safe to ignore
            _ => {}
        }
    }}

    cfg_if! { if #[cfg(feature = "scheduler")] {
        match nr {
            SYS_clone => return call5(regs, |flags, stack, parent_tid, tls, child_tid| {
                handlers::thread::sys_clone(flags, stack, parent_tid, tls, child_tid, &ctx)
            }),
            SYS_exit => return call1(regs, |status| handlers::thread::sys_exit(status)),
            SYS_exit_group => return call1(regs, |status| handlers::thread::sys_exit_group(status)),
            SYS_futex => return call3(regs, |addr, op, val| {
                handlers::thread::sys_futex(addr, op, val, &ctx)
            }),
            SYS_sched_yield => return call0(regs, || handlers::thread::sys_sched_yield(&ctx)),
            SYS_getpid => return call0(regs, || handlers::thread::sys_getpid()),
            SYS_gettid => return call0(regs, || handlers::thread::sys_gettid()),
            SYS_set_tid_address => return call1(regs, |tidptr| handlers::thread::sys_set_tid_address(tidptr)),
            _ => {}
        }
    }}

    cfg_if! { if #[cfg(feature = "vfs")] {
        match nr {
            SYS_openat => return call4(regs, |dirfd, path, flags, mode| {
                handlers::vfs::sys_openat(dirfd, path, flags, mode)
            }),
            SYS_close => return call1(regs, handlers::vfs::sys_close),
            SYS_read => return call3(regs, handlers::vfs::sys_read),
            SYS_write => return call3(regs, handlers::vfs::sys_write),
            SYS_readv => return call3(regs, |fd, iov, iovcnt| {
                handlers::vfs::sys_readv(fd, iov, iovcnt)
            }),
            SYS_writev => return call3(regs, |fd, iov, iovcnt| {
                handlers::vfs::sys_writev(fd, iov, iovcnt)
            }),
            SYS_lseek => return call3(regs, |fd, offset, whence| {
                handlers::vfs::sys_lseek(fd, offset, whence)
            }),
            SYS_ioctl => return call3(regs, |fd, request, arg| {
                handlers::vfs::sys_ioctl(fd, request, arg)
            }),
            SYS_fstat => return call2(regs, handlers::vfs::sys_fstat),
            _ => {}
        }
    }}

    cfg_if! { if #[cfg(feature = "random")] {
        if nr == SYS_getrandom { return call3(regs, |buf, buflen, flags| {
            handlers::random::sys_getrandom(buf, buflen, flags)
        }) }
    }}

    match nr {
        SYS_exit | SYS_exit_group => call1(regs, handlers::info::sys_exit),
        SYS_clock_gettime => call2(regs, |clockid, timespec| {
            handlers::info::sys_clock_gettime(clockid, timespec)
        }),
        SYS_prlimit64 => call4(regs, |pid, res, new_limit, old_limit| {
            handlers::info::sys_prlimit64(pid, res, new_limit, old_limit)
        }),
        SYS_rt_sigaction => call3(regs, |sig, act, oldact| {
            handlers::info::sys_rt_sigaction(sig, act, oldact)
        }),
        SYS_rt_sigprocmask => call3(regs, |how, set, oldset| {
            handlers::info::sys_rt_sigprocmask(how, set, oldset)
        }),
        SYS_rt_sigreturn | SYS_set_robust_list | SYS_get_robust_list => {
            call0(regs, handlers::sys_noop)
        }
        _ => call0(regs, handlers::sys_unsupported),
    }
}
