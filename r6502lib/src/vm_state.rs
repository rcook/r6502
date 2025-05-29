use crate::{Memory, Reg, STACK_BASE};

pub(crate) struct VmState {
    pub(crate) reg: Reg,
    pub(crate) memory: Memory,
}

impl VmState {
    pub(crate) fn push(&mut self, value: u8) {
        let addr = STACK_BASE + self.reg.s as u16;
        self.memory[addr] = value;
        self.reg.s = self.reg.s.wrapping_sub(1);
    }

    pub(crate) fn pull(&mut self) -> u8 {
        self.reg.s = self.reg.s.wrapping_add(1);
        let addr = STACK_BASE + self.reg.s as u16;
        self.memory[addr]
    }
}
