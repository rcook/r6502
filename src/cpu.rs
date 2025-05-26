use crate::{CpuMessage, Flag, Memory, Thunk, STACK_BASE};
use std::sync::mpsc::{Receiver, TryRecvError};

pub(crate) struct Cpu {
    pub(crate) p: u8,
    pub(crate) pc: u16,
    pub(crate) a: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) s: u8,
    pub(crate) memory: Memory,
    rx: Receiver<CpuMessage>,
    thunk: Thunk,
    free_running: bool,
}

impl Cpu {
    pub(crate) fn new(thunk: Thunk, vm_rx: Receiver<CpuMessage>) -> Self {
        Self {
            pc: 0x0000u16,
            p: 0x00u8,
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            s: 0xffu8,
            memory: [0x00u8; 0x10000],
            rx: vm_rx,
            thunk,
            free_running: false,
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

    pub(crate) fn fetch(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub(crate) fn fetch_word(&self, addr: u16) -> u16 {
        let lo = self.fetch(addr);
        let hi = self.fetch(addr + 1);
        Self::make_word(hi, lo)
    }

    pub(crate) fn store(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value
    }

    pub(crate) fn store_word(&mut self, addr: u16, value: u16) {
        let (hi, lo) = Self::split_word(value);
        self.store(addr, lo);
        self.store(addr + 1, hi);
    }

    pub(crate) fn next(&mut self) -> u8 {
        let value = self.fetch(self.pc);
        self.pc += 1;
        value
    }

    pub(crate) fn next_word(&mut self) -> u16 {
        let lo = self.next();
        let hi = self.next();
        Self::make_word(hi, lo)
    }

    pub(crate) fn push(&mut self, value: u8) {
        let addr = STACK_BASE + self.s as u16;
        self.store(addr, value);
        self.s -= 1;
    }

    pub(crate) fn push_word(&mut self, value: u16) {
        let (hi, lo) = Self::split_word(value);
        self.push(hi);
        self.push(lo);
    }

    pub(crate) fn pull(&mut self) -> u8 {
        self.s += 1;
        self.fetch(STACK_BASE + self.s as u16)
    }

    pub(crate) fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        Self::make_word(hi, lo)
    }

    #[allow(unused)]
    pub(crate) fn dump_p(&self) {
        println!(
            "pc={:04X} NV1BDIZC={:08b} a={:02X} x={:02X} y={:02X} s={:02X}",
            self.pc, self.p, self.a, self.x, self.y, self.s,
        )
    }

    #[allow(unused)]
    pub(crate) fn dump_stack(&self) {
        for i in 0..256 {
            println!("{:04X}: {:02X}", STACK_BASE + i, self.fetch(STACK_BASE + i));
        }
    }

    pub(crate) fn write_stdout(&self, c: char) {
        self.thunk.write_stdout(c);
    }

    pub(crate) fn println(&self, s: &str) {
        self.thunk.println(s);
    }

    pub(crate) fn poll(&mut self) -> bool {
        loop {
            if self.free_running {
                match self.rx.try_recv() {
                    Err(TryRecvError::Disconnected) => return false,
                    Err(TryRecvError::Empty) => return true,
                    Ok(CpuMessage::Step) => {}
                    Ok(CpuMessage::Run) => {}
                    Ok(CpuMessage::Break) => self.free_running = false,
                }
            } else {
                match self.rx.recv() {
                    Err(_) => return false,
                    Ok(CpuMessage::Step) => return true,
                    Ok(CpuMessage::Run) => self.free_running = true,
                    Ok(CpuMessage::Break) => {}
                }
            }
        }
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
