use crate::{p_set, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#JMP
// http://www.6502.org/users/obelisk/6502/reference.html#JMP
pub(crate) fn jmp(s: &mut VmState, operand: u16) {
    s.reg.pc = operand;
}

// http://www.6502.org/tutorials/6502opcodes.html#JSR
// http://www.6502.org/users/obelisk/6502/reference.html#JSR
pub(crate) fn jsr(s: &mut VmState, operand: u16) {
    let return_addr = s.reg.pc;
    s.push_word(return_addr.wrapping_sub(1));
    s.reg.pc = operand;
}

// http://www.6502.org/tutorials/6502opcodes.html#RTI
// http://www.6502.org/users/obelisk/6502/reference.html#RTI
pub(crate) fn rti(s: &mut VmState) {
    s.reg.p = P::from_bits(s.pull()).expect("Must succeed");
    p_set!(s.reg, ALWAYS_ONE, true);
    p_set!(s.reg, B, false);
    let return_addr = s.pull_word();
    s.reg.pc = return_addr;
}

// http://www.6502.org/tutorials/6502opcodes.html#RTS
// http://www.6502.org/users/obelisk/6502/reference.html#RTS
pub(crate) fn rts(s: &mut VmState) {
    let return_addr = s.pull_word().wrapping_add(1);
    s.reg.pc = return_addr;
}

#[cfg(test)]
mod tests {
    use crate::ops::{jmp, jsr, rti};
    use crate::{reg, Memory, Reg, RegBuilder, VmState, VmStateBuilder, _p, OSWRCH};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(reg!(0x12, 0x1000), reg!(0x12, 0x0000), 0x1000)]
    fn jmp_basics(#[case] expected_reg: Reg, #[case] reg: Reg, #[case] operand: u16) {
        let mut s = VmState {
            reg,
            memory: Memory::new(),
        };
        jmp(&mut s, operand);
        assert_eq!(expected_reg, s.reg);
    }

    #[test]
    fn jsr_basics() {
        const TARGET_ADDR: u16 = OSWRCH;
        const RETURN_ADDR: u16 = 0x1234;

        let mut s = VmState::default();
        s.reg.pc = RETURN_ADDR;
        jsr(&mut s, TARGET_ADDR);
        assert_eq!(RETURN_ADDR - 1, s.peek_word());
        assert_eq!(TARGET_ADDR, s.reg.pc)
    }

    #[test]
    // cargo run -p r6502validation -- run-json '{ "name": "40 9c 2c", "initial": { "pc": 34673, "s": 110, "a": 162, "x": 129, "y": 126, "p": 99, "ram": [ [34673, 64], [34674, 156], [34675, 44], [366, 152], [367, 156], [368, 170], [369, 101], [26026, 14]]}, "final": { "pc": 26026, "s": 113, "a": 162, "x": 129, "y": 126, "p": 172, "ram": [ [366, 152], [367, 156], [368, 170], [369, 101], [26026, 14], [34673, 64], [34674, 156], [34675, 44]]}, "cycles": [ [34673, 64, "read"], [34674, 156, "read"], [366, 152, "read"], [367, 156, "read"], [368, 170, "read"], [369, 101, "read"]] }'
    fn rti_scenario() -> Result<()> {
        const INITIAL_S: u8 = 0x6e;
        let reg = RegBuilder::default().p(_p!(0x63)).s(INITIAL_S).build()?;
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        s.memory[INITIAL_S as u16] = 0x98;
        s.memory[INITIAL_S as u16 + 1] = 0x9c; // P
        s.memory[INITIAL_S as u16 + 2] = 0xaa; // lo(return_attr)
        s.memory[INITIAL_S as u16 + 3] = 0x65; // hi(return_attr)
        s.reg.pc = 0x8771 + 1;
        rti(&mut s);
        assert_eq!(0x65aa, s.reg.pc);
        assert_eq!(_p!(0xac), s.reg.p); // P & 0b11101111
        assert_eq!(INITIAL_S + 3, s.reg.s);
        Ok(())
    }
}
