use crate::emulator::Cpu;
use crate::emulator::ops::helper::is_neg;
use r6502cpu::p_set;

// http://www.6502.org/tutorials/6502opcodes.html#CMP
// http://www.6502.org/users/obelisk/6502/reference.html#CMP
pub fn cmp(cpu: &mut Cpu, operand: u8) {
    compare_helper(cpu, cpu.reg.a, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPX
// http://www.6502.org/users/obelisk/6502/reference.html#CPX
pub fn cpx(cpu: &mut Cpu, operand: u8) {
    compare_helper(cpu, cpu.reg.x, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#CPY
// http://www.6502.org/users/obelisk/6502/reference.html#CPY
pub fn cpy(cpu: &mut Cpu, operand: u8) {
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
    use crate::emulator::ops::cmp;
    use crate::emulator::{Bus, Cpu};
    use r6502cpu::{_p, InterruptChannel};

    #[test]
    fn basics() {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = 0x10;
        cpu.reg.p = _p!(0b1010_1111);
        cmp(&mut cpu, 0xbb);
        assert_eq!(_p!(0b0010_1100), cpu.reg.p);
    }
}
