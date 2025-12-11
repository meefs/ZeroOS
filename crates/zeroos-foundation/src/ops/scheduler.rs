#[derive(Clone, Copy)]
pub struct SchedulerOps {
    pub yield_now: fn(),
    pub current_tid: fn() -> usize,
    pub thread_count: fn() -> usize,
    pub exit_thread: fn(code: i32) -> !,
    pub clone_thread: fn(
        flags: usize,
        stack: usize,
        parent_tid: usize,
        child_tid: usize,
        tls: usize,
        mepc: usize,
        frame_ptr: usize,
    ) -> isize,
    pub futex_wait: fn(addr: usize, val: i32) -> isize,
    pub futex_wake: fn(addr: usize, count: usize) -> usize,
    pub get_current_a0: fn() -> isize,
    pub update_frame: fn(frame_ptr: usize, mepc: usize),
    pub finish_trap: fn(frame_ptr: usize, mepc_ptr: usize, mepc: usize),
    pub set_tid_address: fn(tidptr: usize) -> usize,
}
