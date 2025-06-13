use crate::emulator::{Cpu, P};

#[derive(Debug, PartialEq)]
pub(crate) enum BranchResult {
    NotTaken,
    Taken,
    TakenCrossPage,
}

impl BranchResult {
    pub(crate) fn compute(cpu: &mut Cpu, offset: u8, p: P, flag_value: bool) -> Self {
        if cpu.reg.p.contains(p) == flag_value {
            // Sign-extend the offset before adding it
            let new_pc = cpu.reg.pc.wrapping_add((offset as i8) as u16);

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
