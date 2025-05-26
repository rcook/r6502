use crate::{Flag, Memory, STACK_BASE};

pub(crate) struct State {
    pub(crate) p: u8,
    pub(crate) pc: u16,
    pub(crate) a: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) s: u8,
    pub(crate) memory: Memory,
    pub(crate) running: bool,
}

impl State {
    pub(crate) fn new() -> Self {
        Self {
            pc: 0x0000u16,
            p: 0x00u8,
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            s: 0xffu8,
            memory: [0x00u8; 0x10000],
            running: false,
        }
    }

    pub(crate) fn get_flag(&self, flag: Flag) -> bool {
        (self.p & flag as u8) != 0x00u8
    }

    pub(crate) fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.p |= flag as u8
        } else {
            self.p &= !(flag as u8)
        }
    }

    pub(crate) fn fetch(&mut self) -> u8 {
        let value = self.memory[self.pc as usize];
        self.pc += 1;
        value
    }

    pub(crate) fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch();
        let hi = self.fetch();
        Self::make_word(hi, lo)
    }

    pub(crate) fn push(&mut self, value: u8) {
        let addr = STACK_BASE + self.s as u16;
        self.memory[addr as usize] = value;
        self.s -= 1;
    }

    pub(crate) fn push_word(&mut self, value: u16) {
        let (hi, lo) = Self::split_word(value);
        self.push(hi);
        self.push(lo);
    }

    pub(crate) fn pull(&mut self) -> u8 {
        self.s += 1;
        let addr = STACK_BASE + self.s as u16;
        let value = self.memory[addr as usize];
        value
    }

    pub(crate) fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        Self::make_word(hi, lo)
    }

    pub(crate) fn dump(&self) -> String {
        format!(
            "pc={:04X} NV1BDIZC={:08b} a={:02X} x={:02X} y={:02X} s={:02X}",
            self.pc, self.p, self.a, self.x, self.y, self.s,
        )
    }

    pub(crate) fn println(&self, _s: &str) {
        //println!("{s}");
    }

    pub(crate) fn stdout(&self, c: char) {
        print!("{c}")
    }

    fn make_word(hi: u8, lo: u8) -> u16 {
        ((hi as u16) << 8) + lo as u16
    }

    fn split_word(value: u16) -> (u8, u8) {
        let hi = (value >> 8) as u8;
        let lo = value as u8;
        (hi, lo)
    }
}
