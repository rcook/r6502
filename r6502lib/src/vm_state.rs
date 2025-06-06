use crate::util::{make_word, split_word};
use crate::{MemoryView, Reg, STACK_BASE};

pub struct VmState<'a> {
    pub reg: Reg,
    pub memory: MemoryView<'a>,
}

impl<'a> VmState<'a> {
    pub fn new(reg: Reg, memory: MemoryView<'a>) -> Self {
        Self { reg, memory }
    }

    pub fn push(&mut self, value: u8) {
        self.set_stack_value(value);
        self.reg.sp = self.reg.sp.wrapping_sub(1);
    }

    #[must_use]
    pub fn pull(&mut self) -> u8 {
        self.reg.sp = self.reg.sp.wrapping_add(1);
        self.get_stack_value()
    }

    pub fn push_word(&mut self, value: u16) {
        let (hi, lo) = split_word(value);
        self.push(hi);
        self.push(lo);
    }

    #[must_use]
    pub fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        make_word(hi, lo)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn peek_word(&self) -> u16 {
        self.peek_back_word(0x00)
    }

    #[must_use]
    pub(crate) fn peek_back_word(&self, offset: u8) -> u16 {
        let stack_addr = (STACK_BASE + self.reg.sp as u16).wrapping_add(offset as u16);
        let hi = self.memory.load(stack_addr.wrapping_add(2));
        let lo = self.memory.load(stack_addr.wrapping_add(1));
        make_word(hi, lo)
    }

    #[must_use]
    fn get_stack_value(&self) -> u8 {
        self.memory
            .load(STACK_BASE.wrapping_add(self.reg.sp as u16))
    }

    fn set_stack_value(&mut self, value: u8) {
        self.memory
            .store(STACK_BASE.wrapping_add(self.reg.sp as u16), value)
    }
}
