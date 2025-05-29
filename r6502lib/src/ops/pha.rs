use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#PHA
// http://www.6502.org/users/obelisk/6502/reference.html#PHA
pub(crate) fn pha(s: &mut VmState) -> Cycles {
    s.push(s.reg.a);
    3
}

#[cfg(test)]
mod tests {
    use crate::ops::pha::pha;
    use crate::{reg, Memory, VmState, P, STACK_BASE};

    #[test]
    fn basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        s.reg.a = 0x56;
        s.memory[STACK_BASE + 0x00ff] = 0x34;
        assert_eq!(0xff, s.reg.s);
        let cycles = pha(&mut s);
        assert_eq!(0xfe, s.reg.s);
        assert_eq!(3, cycles);
        assert_eq!(0x56, s.reg.a);
        assert_eq!(P::default(), s.reg.p);
        assert_eq!(0x56, s.memory[STACK_BASE + 0x00ff])
    }

    #[test]
    fn wraparound() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        for value in 0x00..=0xff {
            let current_s = 0xff - value;
            s.reg.a = value;
            s.memory[STACK_BASE + 0x00ff - value as u16] = 0x00;
            assert_eq!(current_s, s.reg.s);
            _ = pha(&mut s);
            assert_eq!(current_s.wrapping_sub(1), s.reg.s);
            assert_eq!(value, s.memory[STACK_BASE + 0x00ff - value as u16])
        }
    }
}
