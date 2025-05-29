use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#JSR
// http://www.6502.org/users/obelisk/6502/reference.html#JSR
pub(crate) fn jsr(s: &mut VmState, operand: u16) -> Cycles {
    let return_addr = s.reg.pc;
    s.push_word(return_addr - 1);
    s.reg.pc = operand;
    6
}

#[cfg(test)]
mod tests {
    use crate::ops::jsr::jsr;
    use crate::{constants::OSWRCH, VmState};

    #[test]
    fn basics() {
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
