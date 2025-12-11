// Architecture-specific syscall argument extraction
// Re-exports from architecture crates

#[cfg(target_arch = "riscv64")]
extern crate arch_riscv;

#[cfg(target_arch = "riscv64")]
pub use arch_riscv::{
    call0, call1, call2, call3, call4, call5, call6, get_pc, set_return, syscall_number, PtRegs,
};

// Future: When adding x86_64 support:
// extern crate arch_x86;
// pub use arch_x86::{PtRegs, set_return, syscall_number, get_pc, call0, call1, ...};
