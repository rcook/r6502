use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#TXA
// http://www.6502.org/users/obelisk/6502/reference.html#TXA
pub(crate) fn txa(s: &mut VmState) -> Cycles {
    let value = s.reg.x;
    s.reg.a = value;
    set_flags_on_value(s, value);
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::txa::txa;
    use crate::VmState;

    #[test]
    fn basics() {
        let mut s = VmState::default();
        s.reg.a = 0x00;
        s.reg.x = 0x22;
        assert_eq!(2, txa(&mut s));
        assert_eq!(0x22, s.reg.a);
    }
}
