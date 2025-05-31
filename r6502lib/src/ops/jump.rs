use crate::{Cycles, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#JMP
// http://www.6502.org/users/obelisk/6502/reference.html#JMP
pub(crate) fn jmp(s: &mut VmState, operand: u16) -> Cycles {
    s.reg.pc = operand;
    3
}

// http://www.6502.org/tutorials/6502opcodes.html#JSR
// http://www.6502.org/users/obelisk/6502/reference.html#JSR
pub(crate) fn jsr(s: &mut VmState, operand: u16) -> Cycles {
    let return_addr = s.reg.pc;
    s.push_word(return_addr.wrapping_sub(1));
    s.reg.pc = operand;
    6
}

// http://www.6502.org/tutorials/6502opcodes.html#RTI
// http://www.6502.org/users/obelisk/6502/reference.html#RTI
pub(crate) fn rti(s: &mut VmState) -> Cycles {
    s.reg.p = P::from_bits(s.pull()).expect("Must succeed");
    let return_addr = s.pull_word().wrapping_add(1);
    s.reg.pc = return_addr;
    6
}

// http://www.6502.org/tutorials/6502opcodes.html#RTS
// http://www.6502.org/users/obelisk/6502/reference.html#RTS
pub(crate) fn rts(s: &mut VmState) -> Cycles {
    let return_addr = s.pull_word().wrapping_add(1);
    s.reg.pc = return_addr;
    6
}

#[cfg(test)]
mod tests {
    use crate::ops::jump::{jmp, jsr};
    use crate::{reg, Memory, Reg, VmState, OSWRCH};
    use rstest::rstest;

    #[rstest]
    #[case(reg!(0x12, 0x1000), reg!(0x12, 0x0000), 0x1000)]
    fn jmp_basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u16) {
        let mut s = VmState {
            reg,
            memory: Memory::new(),
        };
        let cycles = jmp(&mut s, operand);
        assert_eq!(3, cycles);
        assert_eq!(expected_reg, s.reg);
    }

    #[test]
    fn jsr_basics() {
        const TARGET_ADDR: u16 = OSWRCH;
        const RETURN_ADDR: u16 = 0x1234;

        let mut s = VmState::default();
        s.reg.pc = RETURN_ADDR;
        let cycles = jsr(&mut s, TARGET_ADDR);
        assert_eq!(6, cycles);
        assert_eq!(RETURN_ADDR - 1, s.peek_word());
        assert_eq!(TARGET_ADDR, s.reg.pc)
    }
}
