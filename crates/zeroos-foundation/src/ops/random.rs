#[derive(Clone, Copy)]
pub struct RandomOps {
    pub fill_bytes: fn(buf: *mut u8, len: usize) -> isize,

    pub init_seed: fn(seed: u64),
}

impl RandomOps {
    pub const fn empty() -> Self {
        Self {
            fill_bytes: |_buf, _len| -38, // ENOSYS
            init_seed: |_seed| {},        // No-op by default
        }
    }
}
