extern crate zeroos;

pub use zeroos::arch_riscv::{decode_trap, Exception, PtRegs, Trap};

#[no_mangle]
pub extern "C" fn trap_handler(regs: *mut PtRegs) {
    let regs = unsafe { &mut *regs };

    match decode_trap(regs.mcause) {
        Trap::Exception(Exception::MachineEnvCall)
        | Trap::Exception(Exception::SupervisorEnvCall)
        | Trap::Exception(Exception::UserEnvCall) => {
            regs.mepc += 4; // Skip past ecall instruction

            crate::runtime::handler::handle_syscall(regs, &mut regs.mepc as *mut usize);
        }
        Trap::Exception(Exception::Breakpoint) => {
            regs.mepc += 4; // Skip past ebreak instruction
        }
        Trap::Exception(code) => {
            htif::exit(100 + code as u32);
        }
        Trap::Interrupt(_code) => {}
    }
}
