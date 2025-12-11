#![no_std]
#![recursion_limit = "2048"]

pub mod boot;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod handler;
pub mod trap;

// External functions that platforms/runtimes MUST provide.
extern "C" {
    // Platform hardware initialization.
    fn __platform_bootstrap();

    // Runtime initialization - never returns.
    fn __runtime_bootstrap() -> !;

    // M-mode trap handler.
    pub fn trap_handler(regs: *mut PtRegs);
}

pub use boot::{__bootstrap, _start};

pub use riscv::register::mcause::{Exception, Interrupt, Trap};
pub use trap::{decode_trap, PtRegs};
