use crate::util::{make_word, split_word};
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

    pub(crate) fn push_word(&mut self, value: u16) {
        let (hi, lo) = split_word(value);
        self.push(hi);
        self.push(lo);
    }

    pub(crate) fn pull(&mut self) -> u8 {
        self.reg.s = self.reg.s.wrapping_add(1);
        let addr = STACK_BASE + self.reg.s as u16;
        self.memory[addr]
    }

    #[allow(unused)]
    pub(crate) fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        make_word(hi, lo)
    }
}
