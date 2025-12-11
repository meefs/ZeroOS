#![no_std]

mod stack_builder;

pub use stack_builder::build_gnu_stack;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;
