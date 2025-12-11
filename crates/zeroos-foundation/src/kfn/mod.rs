use cfg_if::cfg_if;

extern "Rust" {
    fn platform_exit(code: i32) -> !;
}

#[inline]
pub fn kexit(code: i32) -> ! {
    unsafe { platform_exit(code) }
}

cfg_if! {
    if #[cfg(feature = "memory")] {
        pub mod memory;
        pub use memory::*;
    }
}

cfg_if! {
    if #[cfg(feature = "scheduler")] {
        pub mod scheduler;
        pub use scheduler::*;
    }
}

cfg_if! {
    if #[cfg(feature = "vfs")] {
        pub mod vfs;
        pub use vfs::*;
    }
}

cfg_if! {
    if #[cfg(feature = "random")] {
        pub mod random;
        pub use random::*;
    }
}
