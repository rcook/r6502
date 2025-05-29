use crate::util::{is_neg, is_zero};
use crate::{set, Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#PLA
// http://www.6502.org/users/obelisk/6502/reference.html#PLA
pub(crate) fn pla(s: &mut VmState) -> Cycles {
    let value = s.pull();
    s.reg.a = value;
    set!(s.reg, N, is_neg(value));
    set!(s.reg, Z, is_zero(value));
    4
}

#[cfg(test)]
mod tests {
    use crate::pha::pha;
    use crate::pla::pla;
    use crate::{reg, Memory, VmState, P};

    #[test]
    fn basics() {
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

        let cycles = pla(&mut s);
        assert_eq!(4, cycles);
        assert_eq!(0x45, s.reg.a);
        assert_eq!(P::empty(), s.reg.p);

        let cycles = pla(&mut s);
        assert_eq!(4, cycles);
        assert_eq!(0xf1, s.reg.a);
        assert_eq!(P::N, s.reg.p);

        let cycles = pla(&mut s);
        assert_eq!(4, cycles);
        assert_eq!(0x00, s.reg.a);
        assert_eq!(P::Z, s.reg.p);
    }
}
