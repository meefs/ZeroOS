extern crate alloc;

pub fn handle_interrupt(_code: usize) {}

#[inline(always)]
pub fn handle_syscall(regs: *mut zeroos::arch::riscv::PtRegs, mepc_ptr: *mut usize) {
    handle_syscall_impl(regs, mepc_ptr)
}

pub fn exit(code: i32) -> ! {
    htif::exit(code as u32);
}

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        #[inline(always)]
        fn handle_syscall_impl(
            regs: *mut zeroos::arch::riscv::PtRegs,
            mepc_ptr: *mut usize,
        ) {
            let regs_ref = unsafe { &*regs };
            let nr = regs_ref.a7;
            let a0 = regs_ref.a0;
            let a1 = regs_ref.a1;
                let a2 = regs_ref.a2;
                let a3 = regs_ref.a3;
                let a4 = regs_ref.a4;
                let _a5 = regs_ref.a5;
                let mepc = regs_ref.mepc;

            match nr as i64 {
                libc::SYS_write
                | libc::SYS_brk
                | libc::SYS_mmap
                | libc::SYS_munmap
                | libc::SYS_mprotect
                | libc::SYS_rt_sigaction
                | libc::SYS_rt_sigprocmask
                | libc::SYS_prlimit64
                | libc::SYS_getrandom
                | libc::SYS_clock_gettime
                | libc::SYS_ioctl
                | libc::SYS_fstat
                | libc::SYS_writev
                | libc::SYS_getpid
                | libc::SYS_gettid
                | libc::SYS_exit
                | libc::SYS_exit_group
                | libc::SYS_set_tid_address
                | libc::SYS_futex
                | libc::SYS_sched_yield
                | libc::SYS_clone
                | 130  // rt_sigaction
                => {
                    zeroos::os::linux::dispatch_syscall(regs);
                }
                _ => {
                    unsafe {
                        (*regs).a0 = -libc::ENOSYS as usize;
                    }
                }
            }
        }
    } else {
        #[inline(always)]
        fn handle_syscall_impl(
            regs: *mut zeroos::arch::riscv::PtRegs,
            _mepc_ptr: *mut usize,
        ) {
            unsafe {
                (*regs).a0 = usize::MAX;  // ENOSYS
            }
        }
    }
}
