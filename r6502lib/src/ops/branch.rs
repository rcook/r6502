use crate::ops::BranchResult;
use crate::{VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#BCC
// http://www.6502.org/users/obelisk/6502/reference.html#BCC
pub(crate) fn bcc(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::C, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BCS
// http://www.6502.org/users/obelisk/6502/reference.html#BCS
pub(crate) fn bcs(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::C, true)
}

// http://www.6502.org/tutorials/6502opcodes.html#BEQ
// http://www.6502.org/users/obelisk/6502/reference.html#BEQ
pub(crate) fn beq(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::Z, true)
}

// http://www.6502.org/tutorials/6502opcodes.html#BMI
// http://www.6502.org/users/obelisk/6502/reference.html#BMI
pub(crate) fn bmi(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::N, true)
}

// http://www.6502.org/tutorials/6502opcodes.html#BNE
// http://www.6502.org/users/obelisk/6502/reference.html#BNE
pub(crate) fn bne(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::Z, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BPL
// http://www.6502.org/users/obelisk/6502/reference.html#BPL
pub(crate) fn bpl(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::N, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BVC
// http://www.6502.org/users/obelisk/6502/reference.html#BVC
pub(crate) fn bvc(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::V, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BVS
// http://www.6502.org/users/obelisk/6502/reference.html#BVS
pub(crate) fn bvs(s: &mut VmState, offset: u8) -> BranchResult {
    BranchResult::compute(s, offset, P::V, true)
}

#[cfg(test)]
mod tests {
    use crate::ops::branch::{bcs, beq};
    use crate::ops::BranchResult;
    use crate::{p_set, VmState};
    use rstest::rstest;

    #[rstest]
    #[case(BranchResult::NotTaken, 0x1000, false, 0x1000, 0x10)]
    #[case(BranchResult::Taken, 0x1010, true, 0x1000, 0x10)]
    #[case(BranchResult::Taken, 0x10e0, true, 0x10f0, 0xf0)]
    #[case(BranchResult::TakenCrossPage, 0x0ff0, true, 0x1000, 0xf0)]
    fn basics(
        #[case] expected_branch_result: BranchResult,
        #[case] expected_pc: u16,
        #[case] flag_value: bool,
        #[case] pc: u16,
        #[case] offset: u8,
    ) {
        let mut s = VmState::default();
        p_set!(s.reg, Z, flag_value);
        s.reg.pc = pc;
        let branch_result = beq(&mut s, offset);
        assert_eq!(expected_branch_result, branch_result);
        assert_eq!(expected_pc, s.reg.pc);
    }

    #[rstest]
    #[case(0x5e47, 0x5e6e, 0xd7, true)]
    fn bcs_scenarios(
        #[case] expected_pc: u16,
        #[case] pc: u16,
        #[case] offset: u8,
        #[case] carry: bool,
    ) {
        let mut s = VmState::default();
        p_set!(s.reg, C, carry);
        s.reg.pc = pc.wrapping_add(2);
        bcs(&mut s, offset);
        assert_eq!(expected_pc, s.reg.pc);
    }
}
