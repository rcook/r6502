use crate::ops::helper::is_neg;
use crate::{p_set, Cpu};

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub(crate) fn cmp(cpu: &mut Cpu, operand: u8) {
    compare_helper(cpu, cpu.reg.a, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub(crate) fn cpx(cpu: &mut Cpu, operand: u8) {
    compare_helper(cpu, cpu.reg.x, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPY
// http://www.6502.org/users/obelisk/6502/reference.html#CPY
pub(crate) fn cpy(cpu: &mut Cpu, operand: u8) {
    compare_helper(cpu, cpu.reg.y, operand);
}

fn compare_helper(cpu: &mut Cpu, register: u8, operand: u8) {
    let (result, overflow) = register.overflowing_sub(operand);
    p_set!(cpu.reg, N, is_neg(result));
    p_set!(cpu.reg, Z, result == 0);
    p_set!(cpu.reg, C, result == 0 || !overflow);
}

#[cfg(test)]
mod tests {
    use crate::ops::cmp;
    use crate::{Bus, Cpu, _p};

    #[test]
    fn basics() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x10;
        cpu.reg.p = _p!(0b10101111);
        cmp(&mut cpu, 0xbb);
        assert_eq!(_p!(0b00101100), cpu.reg.p);
    }
}
