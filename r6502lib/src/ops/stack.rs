use crate::ops::helper::{is_neg, is_zero};
use crate::{p_set, VmState, P};

// http://www.6502.org/tutorials/6502opcodes.html#PHA
// http://www.6502.org/users/obelisk/6502/reference.html#PHA
pub(crate) fn pha(s: &mut VmState) {
    s.push(s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#PHP
// http://www.6502.org/users/obelisk/6502/reference.html#PHP
pub(crate) fn php(s: &mut VmState) {
    s.push(s.reg.p.bits());
}

// http://www.6502.org/tutorials/6502opcodes.html#PLA
// http://www.6502.org/users/obelisk/6502/reference.html#PLA
pub(crate) fn pla(s: &mut VmState) {
    let value = s.pull();
    s.reg.a = value;
    p_set!(s.reg, N, is_neg(value));
    p_set!(s.reg, Z, is_zero(value));
}

// http://www.6502.org/tutorials/6502opcodes.html#PLP
// http://www.6502.org/users/obelisk/6502/reference.html#PLP
pub(crate) fn plp(s: &mut VmState) {
    s.reg.p = P::from_bits(s.pull()).expect("Must be valid");
}

#[cfg(test)]
mod tests {
    use crate::ops::stack::{pha, php, pla, plp};
    use crate::{reg, Memory, VmState, P, STACK_BASE};

    #[test]
    fn pha_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        s.reg.a = 0x56;
        s.memory[STACK_BASE + 0x00ff] = 0x34;
        assert_eq!(0xff, s.reg.s);
        pha(&mut s);
        assert_eq!(0xfe, s.reg.s);
        assert_eq!(0x56, s.reg.a);
        assert_eq!(P::default(), s.reg.p);
        assert_eq!(0x56, s.memory[STACK_BASE + 0x00ff])
    }

    #[test]
    fn pha_wraparound() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        for value in 0x00..=0xff {
            let current_s = 0xff - value;
            s.reg.a = value;
            s.memory[STACK_BASE + 0x00ff - value as u16] = 0x00;
            assert_eq!(current_s, s.reg.s);
            pha(&mut s);
            assert_eq!(current_s.wrapping_sub(1), s.reg.s);
            assert_eq!(value, s.memory[STACK_BASE + 0x00ff - value as u16])
        }
    }

    #[test]
    fn php_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        s.reg.p = P::N | P::D | P::Z;
        php(&mut s);

        s.reg.p = P::V | P::C;
        php(&mut s);

        s.reg.p = P::empty();

        plp(&mut s);
        assert_eq!(P::V | P::C, s.reg.p);

        plp(&mut s);
        assert_eq!(P::N | P::D | P::Z, s.reg.p);
    }

    #[test]
    fn pla_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        s.reg.a = 0x00;
        _ = pha(&mut s);

        s.reg.a = 0xf1;
        _ = pha(&mut s);

        s.reg.a = 0x45;
        _ = pha(&mut s);

        s.reg.a = 0x11;
        s.reg.p = P::empty();
        assert_eq!(0x11, s.reg.a);

        pla(&mut s);
        assert_eq!(0x45, s.reg.a);
        assert_eq!(P::empty(), s.reg.p);

        pla(&mut s);
        assert_eq!(0xf1, s.reg.a);
        assert_eq!(P::N, s.reg.p);

        pla(&mut s);
        assert_eq!(0x00, s.reg.a);
        assert_eq!(P::Z, s.reg.p);
    }
}
