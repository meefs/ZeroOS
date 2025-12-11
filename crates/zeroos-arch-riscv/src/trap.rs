pub use riscv::register::mcause::{Exception, Interrupt, Trap};

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct PtRegs {
    pub ra: usize,  // x1
    pub sp: usize,  // x2
    pub gp: usize,  // x3
    pub tp: usize,  // x4
    pub t0: usize,  // x5
    pub t1: usize,  // x6
    pub t2: usize,  // x7
    pub s0: usize,  // x8 / fp
    pub s1: usize,  // x9
    pub a0: usize,  // x10
    pub a1: usize,  // x11
    pub a2: usize,  // x12
    pub a3: usize,  // x13
    pub a4: usize,  // x14
    pub a5: usize,  // x15
    pub a6: usize,  // x16
    pub a7: usize,  // x17
    pub s2: usize,  // x18
    pub s3: usize,  // x19
    pub s4: usize,  // x20
    pub s5: usize,  // x21
    pub s6: usize,  // x22
    pub s7: usize,  // x23
    pub s8: usize,  // x24
    pub s9: usize,  // x25
    pub s10: usize, // x26
    pub s11: usize, // x27
    pub t3: usize,  // x28
    pub t4: usize,  // x29
    pub t5: usize,  // x30
    pub t6: usize,  // x31

    // Trap-related CSRs (M-mode)
    pub mepc: usize,
    pub mstatus: usize,
    pub mcause: usize,
    pub mtval: usize,
}

#[allow(non_camel_case_types)]
pub type pt_regs = PtRegs;

impl foundation::ArchContext for PtRegs {
    fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            mepc: 0,
            mstatus: 0,
            mcause: 0,
            mtval: 0,
        }
    }
    fn sp(&self) -> usize {
        self.sp
    }
    fn set_sp(&mut self, sp: usize) {
        self.sp = sp;
    }
    fn tp(&self) -> usize {
        self.tp
    }
    fn set_tp(&mut self, tp: usize) {
        self.tp = tp;
    }
    fn return_value(&self) -> usize {
        self.a0
    }
    fn set_return_value(&mut self, val: usize) {
        self.a0 = val;
    }
    fn ra(&self) -> usize {
        self.ra
    }
    fn set_ra(&mut self, ra: usize) {
        self.ra = ra;
    }
    fn gp(&self) -> usize {
        self.gp
    }
    fn set_gp(&mut self, gp: usize) {
        self.gp = gp;
    }
    unsafe fn read_from_ptr(ptr: *const Self) -> Self {
        *ptr
    }
    unsafe fn write_to_ptr(&self, ptr: *mut Self) {
        *ptr = *self;
    }
}

#[inline]
pub fn decode_trap(mcause: usize) -> Trap {
    let is_int = (mcause & (1 << (usize::BITS - 1))) != 0;
    let code = mcause & !(1 << (usize::BITS - 1));
    if is_int {
        match code {
            1 => Trap::Interrupt(Interrupt::SupervisorSoft),
            3 => Trap::Interrupt(Interrupt::MachineSoft),
            5 => Trap::Interrupt(Interrupt::SupervisorTimer),
            7 => Trap::Interrupt(Interrupt::MachineTimer),
            9 => Trap::Interrupt(Interrupt::SupervisorExternal),
            11 => Trap::Interrupt(Interrupt::MachineExternal),
            _ => Trap::Interrupt(Interrupt::Unknown),
        }
    } else {
        match code {
            0 => Trap::Exception(Exception::InstructionMisaligned),
            1 => Trap::Exception(Exception::InstructionFault),
            2 => Trap::Exception(Exception::IllegalInstruction),
            3 => Trap::Exception(Exception::Breakpoint),
            4 => Trap::Exception(Exception::LoadMisaligned),
            5 => Trap::Exception(Exception::LoadFault),
            6 => Trap::Exception(Exception::StoreMisaligned),
            7 => Trap::Exception(Exception::StoreFault),
            8 => Trap::Exception(Exception::UserEnvCall),
            9 => Trap::Exception(Exception::SupervisorEnvCall),
            11 => Trap::Exception(Exception::MachineEnvCall),
            12 => Trap::Exception(Exception::InstructionPageFault),
            13 => Trap::Exception(Exception::LoadPageFault),
            15 => Trap::Exception(Exception::StorePageFault),
            _ => Trap::Exception(Exception::Unknown),
        }
    }
}
