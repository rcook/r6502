use crate::util::{make_word, split_word};
use crate::{Memory, Reg, STACK_BASE};

#[derive(Default)]
pub(crate) struct VmState {
    pub(crate) reg: Reg,
    pub(crate) memory: Memory,
}

impl VmState {
    pub(crate) fn push(&mut self, value: u8) {
        self.set_stack_value(value);
        self.reg.s = self.reg.s.wrapping_sub(1);
    }

    pub(crate) fn pull(&mut self) -> u8 {
        self.reg.s = self.reg.s.wrapping_add(1);
        self.get_stack_value()
    }

    pub(crate) fn push_word(&mut self, value: u16) {
        let (hi, lo) = split_word(value);
        self.push(hi);
        self.push(lo);
    }

    #[allow(unused)]
    pub(crate) fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        make_word(hi, lo)
    }

    #[allow(unused)]
    pub(crate) fn peek(&self) -> u8 {
        let stack_addr = STACK_BASE + self.reg.s as u16;
        self.memory[stack_addr.wrapping_add(1)]
    }

    pub(crate) fn peek_word(&self) -> u16 {
        let stack_addr = STACK_BASE + self.reg.s as u16;
        let hi = self.memory[stack_addr.wrapping_add(2)];
        let lo = self.memory[stack_addr.wrapping_add(1)];
        make_word(hi, lo)
    }

    fn get_stack_value(&self) -> u8 {
        self.memory[STACK_BASE + self.reg.s as u16]
    }

    fn set_stack_value(&mut self, value: u8) {
        self.memory[STACK_BASE + self.reg.s as u16] = value;
    }
}
