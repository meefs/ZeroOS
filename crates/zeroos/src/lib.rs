#![no_std]

pub use foundation;

#[cfg(any(
    feature = "alloc-linked-list",
    feature = "alloc-buddy",
    feature = "alloc-bump"
))]
pub use foundation::register_memory;
pub use zeroos_macros as macros;

#[cfg(feature = "vfs")]
pub use foundation::register_vfs;

#[cfg(feature = "random")]
pub use foundation::register_random;

#[cfg(feature = "scheduler")]
pub use foundation::register_scheduler;

#[cfg(feature = "arch-riscv")]
pub extern crate arch_riscv;

#[cfg(target_os = "none")]
extern crate runtime_nostd;

#[cfg(feature = "runtime-musl")]
extern crate runtime_musl;

#[cfg(feature = "runtime-gnu")]
extern crate runtime_gnu;

#[cfg(feature = "libunwind")]
extern crate libunwind;

zeroos_macros::require_at_most_one_feature!("alloc-linked-list", "alloc-buddy", "alloc-bump");
zeroos_macros::require_at_most_one_feature!("scheduler");

pub mod arch {
    #[cfg(feature = "arch-riscv")]
    pub use arch_riscv as riscv;
}

#[cfg(feature = "scheduler")]
pub mod scheduler {
    pub use scheduler::*;
}

pub mod os {
    #[cfg(feature = "os-linux")]
    pub mod linux {
        pub use os_linux::*;
    }
}

#[cfg(feature = "vfs")]
pub mod vfs {
    pub use vfs::*;

    pub mod devices {
        #[cfg(feature = "vfs-device-console")]
        pub use device_console as console;

        #[cfg(feature = "vfs-device-null")]
        pub use device_null as null;

        #[cfg(feature = "vfs-device-urandom")]
        pub use device_urandom as urandom;

        #[cfg(feature = "vfs-device-zero")]
        pub use device_zero as zero;
    }
}

#[cfg(any(feature = "rng-lcg", feature = "rng-chacha"))]
pub mod rng {
    pub use rng::*;
}
#[cfg(all(
    target_os = "none",
    any(
        feature = "alloc-linked-list",
        feature = "alloc-buddy",
        feature = "alloc-bump"
    )
))]
pub use runtime_nostd::alloc;

pub fn initialize() {
    #[cfg(feature = "alloc-linked-list")]
    foundation::register_memory(allocator_linked_list::LINKED_LIST_ALLOCATOR_OPS);

    #[cfg(feature = "alloc-buddy")]
    foundation::register_memory(allocator_buddy::BUDDY_ALLOCATOR_OPS);

    #[cfg(feature = "alloc-bump")]
    foundation::register_memory(allocator_bump::BUMP_ALLOCATOR_OPS);

    #[cfg(feature = "scheduler")]
    {
        scheduler::Scheduler::init();
        foundation::register_scheduler(scheduler::create_scheduler_ops());
    }

    #[cfg(feature = "vfs")]
    foundation::register_vfs(vfs::VFS_OPS);

    #[cfg(feature = "rng-lcg")]
    foundation::register_random(rng::LCG_RNG_OPS);

    #[cfg(feature = "rng-chacha")]
    foundation::register_random(rng::CHACHA_RNG_OPS);
}
