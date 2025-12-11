use super::{errno, HandlerContext};
use crate::kfn;

fn mepc_ptr(ctx: &HandlerContext) -> usize {
    if ctx.frame_ptr == 0 {
        0
    } else {
        ctx.frame_ptr + 248
    }
}

pub fn sys_clone(
    flags: usize,
    stack: usize,
    parent_tid: usize,
    tls: usize,       // a3: tls
    child_tid: usize, // a4: child_tid
    ctx: &HandlerContext,
) -> isize {
    kfn::clone_thread(
        flags,
        stack,
        parent_tid,
        child_tid,
        tls,
        ctx.mepc,
        ctx.frame_ptr,
    );
    kfn::get_current_a0()
}

pub fn sys_exit(status: usize) -> isize {
    kfn::exit_thread(status as i32);
}

pub fn sys_exit_group(status: usize) -> isize {
    kfn::exit_thread(status as i32);
}

pub fn sys_futex(addr: usize, op: usize, val: usize, ctx: &HandlerContext) -> isize {
    use debug::writeln; // Keep minimal logging for futex as requested
    let tid = kfn::current_tid();
    writeln!(
        "sys_futex: tid={} addr={:#x} op={} val={}",
        tid,
        addr, op, val
    );

    const FUTEX_WAIT: i32 = 0;
    const FUTEX_WAKE: i32 = 1;
    const FUTEX_WAIT_BITSET: i32 = 9;
    const FUTEX_WAKE_BITSET: i32 = 10;
    const FUTEX_PRIVATE_FLAG: i32 = 128;
    let op_masked = (op as i32) & !FUTEX_PRIVATE_FLAG;
    let result = match op_masked {
        FUTEX_WAIT | FUTEX_WAIT_BITSET => {
            let current = unsafe { *(addr as *const i32) };
            if current != val as i32 {
                writeln!(
                    "sys_futex: tid={} addr={:#x} WAIT fail current={} != val={}",
                    tid,
                    addr, current, val
                );
                -errno::EAGAIN
            } else {
                writeln!("sys_futex: tid={} addr={:#x} WAIT sleeping...", tid, addr);
                kfn::futex_wait(addr, val as i32)
            }
        }
        FUTEX_WAKE | FUTEX_WAKE_BITSET => {
            let ret = kfn::futex_wake(addr, val) as isize;
            writeln!("sys_futex: tid={} addr={:#x} WAKE count={}", tid, addr, ret);
            ret
        }
        _ => -errno::EINVAL,
    };
    writeln!("sys_futex: tid={} result={}", tid, result);
    kfn::get_current_a0()
}

pub fn sys_sched_yield(ctx: &HandlerContext) -> isize {
    kfn::sched_yield();
    kfn::get_current_a0()
}

pub fn sys_getpid() -> isize {
    1
}

pub fn sys_gettid() -> isize {
    kfn::current_tid() as isize
}

pub fn sys_set_tid_address(tidptr: usize) -> isize {
    kfn::set_tid_address(tidptr) as isize
}
