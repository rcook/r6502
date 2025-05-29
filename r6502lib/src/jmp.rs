use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#JMP
// http://www.6502.org/users/obelisk/6502/reference.html#JMP
pub(crate) fn jmp(s: &mut VmState, operand: u16) -> Cycles {
    s.reg.pc = operand;
    3
}

#[cfg(test)]
mod tests {
    use crate::jmp::jmp;
    use crate::{reg, Memory, Reg, VmState};
    use rstest::rstest;

    #[rstest]
    #[case(reg!(0x12, 0x1000), reg!(0x12, 0x0000), 0x1000)]
    fn basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u16) {
        let mut s = VmState {
            reg,
            memory: Memory::new(),
        };
        let cycles = jmp(&mut s, operand);
        assert_eq!(3, cycles);
        assert_eq!(expected_reg, s.reg);
    }
}
