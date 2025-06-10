use crate::ops::helper::is_neg;
use crate::{p_set, Cpu};

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(state: &mut Cpu, operand: u8) {
    compare_helper(state, state.reg.a, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub(crate) fn cpx(state: &mut Cpu, operand: u8) {
    compare_helper(state, state.reg.x, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPY
// http://www.6502.org/users/obelisk/6502/reference.html#CPY
pub(crate) fn cpy(state: &mut Cpu, operand: u8) {
    compare_helper(state, state.reg.y, operand);
}

fn compare_helper(state: &mut Cpu, register: u8, operand: u8) {
    let (result, overflow) = register.overflowing_sub(operand);
    p_set!(state.reg, N, is_neg(result));
    p_set!(state.reg, Z, result == 0);
    p_set!(state.reg, C, result == 0 || !overflow);
}

#[cfg(test)]
mod tests {
    use crate::ops::cmp;
    use crate::{Cpu, DummyMonitor, Memory, _p};
    use anyhow::Result;

    #[test]
    fn basics() -> Result<()> {
        let memory = Memory::default();
        let mut cpu = Cpu::new(crate::Reg::default(), memory.view(), Box::new(DummyMonitor));
        cpu.reg.a = 0x10;
        cpu.reg.p = _p!(0b10101111);
        cmp(&mut cpu, 0xbb);
        assert_eq!(_p!(0b00101100), cpu.reg.p);
        Ok(())
    }
}
