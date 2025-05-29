use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#PHP
// http://www.6502.org/users/obelisk/6502/reference.html#PHP
pub(crate) fn php(s: &mut VmState) -> Cycles {
    s.push(s.reg.p.bits());
    3
}

#[cfg(test)]
mod tests {
    use crate::ops::php::php;
    use crate::ops::plp::plp;
    use crate::{reg, Memory, VmState, P};

    #[test]
    fn basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        s.reg.p = P::N | P::D | P::Z;
        let cycles = php(&mut s);
        assert_eq!(3, cycles);

        s.reg.p = P::V | P::C;
        let cycles = php(&mut s);
        assert_eq!(3, cycles);

        s.reg.p = P::empty();

        let cycles = plp(&mut s);
        assert_eq!(4, cycles);
        assert_eq!(P::V | P::C, s.reg.p);

        let cycles = plp(&mut s);
        assert_eq!(4, cycles);
        assert_eq!(P::N | P::D | P::Z, s.reg.p);
    }
}
