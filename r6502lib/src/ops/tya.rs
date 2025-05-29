use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#TYA
// http://www.6502.org/users/obelisk/6502/reference.html#TYA
pub(crate) fn tya(s: &mut VmState) -> Cycles {
    let value = s.reg.y;
    s.reg.a = value;
    set_flags_on_value(s, value);
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::tya::tya;
    use crate::VmState;

    #[test]
    fn basics() {
        let mut s = VmState::default();
        s.reg.a = 0x00;
        s.reg.y = 0x22;
        assert_eq!(2, tya(&mut s));
        assert_eq!(0x22, s.reg.a);
    }
}
