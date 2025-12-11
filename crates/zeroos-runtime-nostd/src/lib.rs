#![no_std]

#[cfg(feature = "memory")]
pub mod alloc;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::__runtime_bootstrap;
