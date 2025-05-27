use crate::{make_word, split_word, Flag, Memory, RegisterFile, STACK_BASE};

pub(crate) struct MachineState {
    pub(crate) reg: RegisterFile,
    pub(crate) memory: Memory,
}

impl MachineState {
    pub(crate) fn new() -> Self {
        Self {
            reg: RegisterFile::new(),
            memory: [0x00u8; 0x10000],
        }
    }

    pub(crate) fn get_flag(&self, flag: Flag) -> bool {
        (self.reg.p & flag as u8) != 0x00u8
    }

    pub(crate) fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.reg.p |= flag as u8
        } else {
            self.reg.p &= !(flag as u8)
        }
    }

    pub(crate) fn set_flags_for(&mut self, value: u8) {
        self.set_flag(Flag::N, value >= 0x80u8);
        self.set_flag(Flag::Z, value == 0);
    }

    pub(crate) fn fetch(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn fetch_word(&self, addr: u16) -> u16 {
        let lo = self.fetch(addr);
        let hi = self.fetch(addr + 1);
        make_word(hi, lo)
    }

    pub(crate) fn store(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value
    }

    pub(crate) fn store_word(&mut self, addr: u16, value: u16) {
        let (hi, lo) = split_word(value);
        self.store(addr, lo);
        self.store(addr + 1, hi);
    }

    #[allow(unused)]
    pub(crate) fn next(&mut self) -> u8 {
        let value = self.fetch(self.reg.pc);
        self.reg.pc += 1;
        value
    }

    #[allow(unused)]
    pub(crate) fn next_word(&mut self) -> u16 {
        let lo = self.next();
        let hi = self.next();
        make_word(hi, lo)
    }

    pub(crate) fn push(&mut self, value: u8) {
        let addr = STACK_BASE + self.reg.s as u16;
        self.store(addr, value);
        self.reg.s -= 1;
    }

    pub(crate) fn push_word(&mut self, value: u16) {
        let (hi, lo) = split_word(value);
        self.push(hi);
        self.push(lo);
    }

    pub(crate) fn pull(&mut self) -> u8 {
        self.reg.s += 1;
        self.fetch(STACK_BASE + self.reg.s as u16)
    }

    pub(crate) fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        make_word(hi, lo)
    }
}
