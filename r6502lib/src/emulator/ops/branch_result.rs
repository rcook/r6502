use crate::emulator::Cpu;
use r6502core::num::SignExtend;
use r6502cpu::P;

#[derive(Debug, PartialEq)]
pub enum BranchResult {
    NotTaken,
    Taken,
    TakenCrossPage,
}

impl BranchResult {
    pub fn compute(cpu: &mut Cpu, offset: u8, p: P, flag_value: bool) -> Self {
        if cpu.reg.p.contains(p) == flag_value {
            let new_pc = cpu.reg.pc.wrapping_add(u16::sign_extend(offset));

            let current_page = cpu.reg.pc >> 8;
            let new_page = new_pc >> 8;

            cpu.reg.pc = new_pc;

            if new_page == current_page {
                Self::Taken
            } else {
                Self::TakenCrossPage
            }
        } else {
            Self::NotTaken
        }
    }
}
