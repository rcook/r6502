use super::helper::set_flags_on_value;
use crate::VmState;

// http://www.6502.org/tutorials/6502opcodes.html#DEX
// http://www.6502.org/users/obelisk/6502/reference.html#DEX
pub(crate) fn dex(s: &mut VmState) {
    let value = s.reg.x.wrapping_sub(1);
    s.reg.x = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#DEY
// http://www.6502.org/users/obelisk/6502/reference.html#DEY
pub(crate) fn dey(s: &mut VmState) {
    let value = s.reg.y.wrapping_sub(1);
    s.reg.y = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#INX
// http://www.6502.org/users/obelisk/6502/reference.html#INX
pub(crate) fn inx(s: &mut VmState) {
    let value = s.reg.x.wrapping_add(1);
    s.reg.x = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#INY
// http://www.6502.org/users/obelisk/6502/reference.html#INY
pub(crate) fn iny(s: &mut VmState) {
    let value = s.reg.y.wrapping_add(1);
    s.reg.y = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TAX
// http://www.6502.org/users/obelisk/6502/reference.html#TAX
pub(crate) fn tax(s: &mut VmState) {
    let value = s.reg.a;
    s.reg.x = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TAY
// http://www.6502.org/users/obelisk/6502/reference.html#TAY
pub(crate) fn tay(s: &mut VmState) {
    let value = s.reg.a;
    s.reg.y = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TSX
// http://www.6502.org/users/obelisk/6502/reference.html#TSX
pub(crate) fn tsx(s: &mut VmState) {
    let value = s.reg.sp;
    s.reg.x = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TXA
// http://www.6502.org/users/obelisk/6502/reference.html#TXA
pub(crate) fn txa(s: &mut VmState) {
    let value = s.reg.x;
    s.reg.a = value;
    set_flags_on_value(s, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TXS
// http://www.6502.org/users/obelisk/6502/reference.html#TXS
pub(crate) fn txs(s: &mut VmState) {
    let value = s.reg.x;
    s.reg.sp = value;
}

// http://www.6502.org/tutorials/6502opcodes.html#TYA
// http://www.6502.org/users/obelisk/6502/reference.html#TYA
pub(crate) fn tya(s: &mut VmState) {
    let value = s.reg.y;
    s.reg.a = value;
    set_flags_on_value(s, value);
}

#[cfg(test)]
mod tests {
    use crate::ops::register::{tax, tay, txa, tya};
    use crate::{Memory, Reg, VmState};

    #[test]
    fn tax_basics() {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.a = 0x22;
        s.reg.x = 0x00;
        tax(&mut s);
        assert_eq!(0x22, s.reg.x);
    }

    #[test]
    fn tay_basics() {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.a = 0x22;
        s.reg.y = 0x00;
        tay(&mut s);
        assert_eq!(0x22, s.reg.y);
    }

    #[test]
    fn txa_basics() {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.a = 0x00;
        s.reg.x = 0x22;
        txa(&mut s);
        assert_eq!(0x22, s.reg.a);
    }

    #[test]
    fn tya_basics() {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.a = 0x00;
        s.reg.y = 0x22;
        tya(&mut s);
        assert_eq!(0x22, s.reg.a);
    }
}
