use super::helper::set_flags_on_value;
use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#TAX
// http://www.6502.org/users/obelisk/6502/reference.html#TAX
pub(crate) fn tax(s: &mut VmState) -> Cycles {
    let value = s.reg.a;
    s.reg.x = value;
    set_flags_on_value(s, value);
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::tax::tax;
    use crate::VmState;

    #[test]
    fn basics() {
        let mut s = VmState::default();
        s.reg.a = 0x22;
        s.reg.x = 0x00;
        assert_eq!(2, tax(&mut s));
        assert_eq!(0x22, s.reg.x);
    }
}
