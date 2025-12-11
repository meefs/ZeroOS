use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "memory")] {
        pub mod memory;
        pub use memory::MemoryOps;
    }
}

cfg_if! {
    if #[cfg(feature = "scheduler")] {
        pub mod scheduler;
        pub use scheduler::SchedulerOps;
    }
}

cfg_if! {
    if #[cfg(feature = "vfs")] {
        pub mod vfs;
        pub use vfs::VfsOps;
    }
}

cfg_if! {
    if #[cfg(feature = "random")] {
        pub mod random;
        pub use random::RandomOps;
    }
}
