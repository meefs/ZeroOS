#[inline]
pub fn sched_yield() {
    unsafe { (crate::KERNEL.scheduler.yield_now)() }
}
#[inline]
pub fn current_tid() -> usize {
    unsafe { (crate::KERNEL.scheduler.current_tid)() }
}
#[inline]
pub fn thread_count() -> usize {
    unsafe { (crate::KERNEL.scheduler.thread_count)() }
}
#[inline]
pub fn exit_thread(code: i32) -> ! {
    unsafe { (crate::KERNEL.scheduler.exit_thread)(code) }
}

#[inline]
pub fn clone_thread(
    flags: usize,
    stack: usize,
    parent_tid: usize,
    child_tid: usize,
    tls: usize,
    mepc: usize,
    frame_ptr: usize,
) -> isize {
    unsafe {
        (crate::KERNEL.scheduler.clone_thread)(
            flags, stack, parent_tid, child_tid, tls, mepc, frame_ptr,
        )
    }
}

#[inline]
pub fn futex_wait(addr: usize, val: i32) -> isize {
    unsafe { (crate::KERNEL.scheduler.futex_wait)(addr, val) }
}
#[inline]
pub fn futex_wake(addr: usize, count: usize) -> usize {
    unsafe { (crate::KERNEL.scheduler.futex_wake)(addr, count) }
}
#[inline]
pub fn get_current_a0() -> isize {
    unsafe { (crate::KERNEL.scheduler.get_current_a0)() }
}
#[inline]
pub fn update_frame(frame_ptr: usize, mepc: usize) {
    unsafe { (crate::KERNEL.scheduler.update_frame)(frame_ptr, mepc) }
}
#[inline]
pub fn finish_trap(frame_ptr: usize, mepc_ptr: usize, mepc: usize) {
    unsafe { (crate::KERNEL.scheduler.finish_trap)(frame_ptr, mepc_ptr, mepc) }
}
#[inline]
pub fn set_tid_address(tidptr: usize) -> usize {
    unsafe { (crate::KERNEL.scheduler.set_tid_address)(tidptr) }
}
