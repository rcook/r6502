use crate::P;
use crate::emulator::Cpu;
use crate::emulator::ops::BranchResult;

// http://www.6502.org/tutorials/6502opcodes.html#BCC
// http://www.6502.org/users/obelisk/6502/reference.html#BCC
pub fn bcc(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::C, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BCS
// http://www.6502.org/users/obelisk/6502/reference.html#BCS
pub fn bcs(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::C, true)
}

// http://www.6502.org/tutorials/6502opcodes.html#BEQ
// http://www.6502.org/users/obelisk/6502/reference.html#BEQ
pub fn beq(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::Z, true)
}

// http://www.6502.org/tutorials/6502opcodes.html#BMI
// http://www.6502.org/users/obelisk/6502/reference.html#BMI
pub fn bmi(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::N, true)
}

// http://www.6502.org/tutorials/6502opcodes.html#BNE
// http://www.6502.org/users/obelisk/6502/reference.html#BNE
pub fn bne(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::Z, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BPL
// http://www.6502.org/users/obelisk/6502/reference.html#BPL
pub fn bpl(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::N, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BVC
// http://www.6502.org/users/obelisk/6502/reference.html#BVC
pub fn bvc(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::V, false)
}

// http://www.6502.org/tutorials/6502opcodes.html#BVS
// http://www.6502.org/users/obelisk/6502/reference.html#BVS
pub fn bvs(cpu: &mut Cpu, offset: u8) -> BranchResult {
    BranchResult::compute(cpu, offset, P::V, true)
}

#[cfg(test)]
mod tests {
    use crate::emulator::ops::BranchResult;
    use crate::emulator::ops::branch::{bcs, beq};
    use crate::emulator::{Bus, Cpu};
    use crate::{InterruptChannel, p_set};
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
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        p_set!(cpu.reg, Z, flag_value);
        cpu.reg.pc = pc;
        let branch_result = beq(&mut cpu, offset);
        assert_eq!(expected_branch_result, branch_result);
        assert_eq!(expected_pc, cpu.reg.pc);
    }

    #[rstest]
    #[case(0x5e47, 0x5e6e, 0xd7, true)]
    fn bcs_scenarios(
        #[case] expected_pc: u16,
        #[case] pc: u16,
        #[case] offset: u8,
        #[case] carry: bool,
    ) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        p_set!(cpu.reg, C, carry);
        cpu.reg.pc = pc.wrapping_add(2);
        bcs(&mut cpu, offset);
        assert_eq!(expected_pc, cpu.reg.pc);
    }
}
