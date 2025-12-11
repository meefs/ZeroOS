use crate::trap::PtRegs;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "riscv64")] {
        zeroos_macros::define_register_helpers!(crate::PtRegs, "sd", "ld");
    } else if #[cfg(target_arch = "riscv32")] {
        zeroos_macros::define_register_helpers!(crate::PtRegs, "sw", "lw");
    } else {
    }
}

use core::arch::global_asm;

mod imp {
    use super::*;
    use zeroos_macros::asm_block;

    #[unsafe(naked)]
    #[no_mangle]
    pub unsafe extern "C" fn save_regs() -> *mut PtRegs {
        asm_block!(
            "addi sp, sp, -{PTREGS_SIZE}",
            store!(t0),
            "addi t0, sp, {PTREGS_SIZE}",
            "csrr t1, mscratch",
            store!(t1, ra),  // Save user's ra to PtRegs.ra
            store!(t0, sp),
            store!(gp),
            store!(tp),
            store!(t1),
            store!(t2),
            store!(s0),
            store!(s1),
            store!(a0),
            store!(a1),
            store!(a2),
            store!(a3),
            store!(a4),
            store!(a5),
            store!(a6),
            store!(a7),
            store!(s2),
            store!(s3),
            store!(s4),
            store!(s5),
            store!(s6),
            store!(s7),
            store!(s8),
            store!(s9),
            store!(s10),
            store!(s11),
            store!(t3),
            store!(t4),
            store!(t5),
            store!(t6),
            "csrr t0, mepc",
            "csrr t1, mstatus",
            "csrr t2, mcause",
            "csrr t3, mtval",
            store!(t0, mepc),
            store!(t1, mstatus),
            store!(t2, mcause),
            store!(t3, mtval),
            "mv a0, sp",
            "ret",
            PTREGS_SIZE = const core::mem::size_of::<PtRegs>(),
        );
    }

    #[unsafe(naked)]
    #[no_mangle]
    pub unsafe extern "C" fn restore_regs(regs: *mut PtRegs) -> ! {
        asm_block!(
            "mv s0, a0",
            load!(t0, mepc, s0),
            load!(t1, mstatus, s0),
            "csrw mepc, t0",
            "csrw mstatus, t1",
            load!(ra, ra, s0),
            load!(gp, gp, s0),
            load!(tp, tp, s0),
            load!(t0, t0, s0),
            load!(t1, t1, s0),
            load!(t2, t2, s0),
            load!(s1, s1, s0),
            load!(a0, a0, s0),
            load!(a1, a1, s0),
            load!(a2, a2, s0),
            load!(a3, a3, s0),
            load!(a4, a4, s0),
            load!(a5, a5, s0),
            load!(a6, a6, s0),
            load!(a7, a7, s0),
            load!(s2, s2, s0),
            load!(s3, s3, s0),
            load!(s4, s4, s0),
            load!(s5, s5, s0),
            load!(s6, s6, s0),
            load!(s7, s7, s0),
            load!(s8, s8, s0),
            load!(s9, s9, s0),
            load!(s10, s10, s0),
            load!(s11, s11, s0),
            load!(t3, t3, s0),
            load!(t4, t4, s0),
            load!(t5, t5, s0),
            load!(t6, t6, s0),
            load!(sp, sp, s0),
            load!(s0, s0, s0),
            "mret",
        );
    }

    #[unsafe(naked)]
    #[no_mangle]
    pub unsafe extern "C" fn _default_trap_handler() {
        asm_block!(
            "csrw mscratch, ra",
            "call {save_regs}",
            "mv s0, a0",
            "call {trap_handler}",
            "mv a0, s0",
            "tail {restore_regs}",
            save_regs = sym save_regs,
            restore_regs = sym restore_regs,
            trap_handler = sym crate::trap_handler,
        );
    }
}

pub use imp::{_default_trap_handler, restore_regs, save_regs};

global_asm!(
    ".align 2",  // 4-byte alignment required for mtvec
    ".weak _trap_handler",
    ".type  _trap_handler, @function",
    "_trap_handler:",
    "j {default}",
    default = sym imp::_default_trap_handler,
);
