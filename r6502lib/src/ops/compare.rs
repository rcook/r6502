use crate::ops::helper::is_neg;
use crate::{p_set, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(s: &mut VmState, operand: u8) {
    compare_helper(s, s.reg.a, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub(crate) fn cpx(s: &mut VmState, operand: u8) {
    compare_helper(s, s.reg.x, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPY
// http://www.6502.org/users/obelisk/6502/reference.html#CPY
pub(crate) fn cpy(s: &mut VmState, operand: u8) {
    compare_helper(s, s.reg.y, operand);
}

fn compare_helper(s: &mut VmState, register: u8, operand: u8) {
    let (result, overflow) = register.overflowing_sub(operand);
    p_set!(s.reg, N, is_neg(result));
    p_set!(s.reg, Z, result == 0);
    p_set!(s.reg, C, result == 0 || !overflow);
}

#[cfg(test)]
mod tests {
    use crate::ops::cmp;
    use crate::{Memory, Reg, VmState, _p};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.a = 0x10;
        s.reg.p = _p!(0b10101111);
        cmp(&mut s, 0xbb);
        assert_eq!(_p!(0b00101100), s.reg.p);
        Ok(())
    }
}
