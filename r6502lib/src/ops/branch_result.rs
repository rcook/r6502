use crate::{CpuState, P};

#[derive(Debug, PartialEq)]
pub(crate) enum BranchResult {
    NotTaken,
    Taken,
    TakenCrossPage,
}

impl BranchResult {
    pub(crate) fn compute(state: &mut CpuState, offset: u8, p: P, flag_value: bool) -> Self {
        if state.reg.p.contains(p) == flag_value {
            // Sign-extend the offset before adding it
            let new_pc = state.reg.pc.wrapping_add((offset as i8) as u16);

            let current_page = state.reg.pc >> 8;
            let new_page = new_pc >> 8;

            state.reg.pc = new_pc;

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
