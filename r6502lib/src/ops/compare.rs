use crate::ops::helper::is_neg;
use crate::{p_set, CpuState};

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(state: &mut CpuState, operand: u8) {
    compare_helper(state, state.reg.a, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub(crate) fn cpx(state: &mut CpuState, operand: u8) {
    compare_helper(state, state.reg.x, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPY
// http://www.6502.org/users/obelisk/6502/reference.html#CPY
pub(crate) fn cpy(state: &mut CpuState, operand: u8) {
    compare_helper(state, state.reg.y, operand);
}

fn compare_helper(state: &mut CpuState, register: u8, operand: u8) {
    let (result, overflow) = register.overflowing_sub(operand);
    p_set!(state.reg, N, is_neg(result));
    p_set!(state.reg, Z, result == 0);
    p_set!(state.reg, C, result == 0 || !overflow);
}

#[cfg(test)]
mod tests {
    use crate::ops::cmp;
    use crate::{CpuState, Memory, Reg, _p};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let memory = Memory::default();
        let mut state = CpuState::new(Reg::default(), memory.view());
        state.reg.a = 0x10;
        state.reg.p = _p!(0b10101111);
        cmp(&mut state, 0xbb);
        assert_eq!(_p!(0b00101100), state.reg.p);
        Ok(())
    }
}
