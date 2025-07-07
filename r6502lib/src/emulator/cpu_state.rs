use crate::_p;
use crate::emulator::{Cpu, TotalCycles};

#[derive(Debug)]
pub struct CpuState {
    pub pc: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub p: u8,
    pub total_cycles: TotalCycles,
}

impl CpuState {
    #[must_use]
    pub fn new(cpu: &Cpu) -> Self {
        Self {
            pc: cpu.reg.pc,
            a: cpu.reg.a,
            x: cpu.reg.x,
            y: cpu.reg.y,
            sp: cpu.reg.sp,
            p: cpu.reg.p.bits(),
            total_cycles: cpu.total_cycles,
        }
    }

    pub fn apply_to(&self, cpu: &mut Cpu) {
        cpu.reg.pc = self.pc;
        cpu.reg.a = self.a;
        cpu.reg.x = self.x;
        cpu.reg.y = self.y;
        cpu.reg.sp = self.sp;
        cpu.reg.p = _p!(self.p);
        cpu.total_cycles = self.total_cycles;
    }
}
