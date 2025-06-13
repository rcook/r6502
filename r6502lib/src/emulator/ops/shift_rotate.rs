use crate::emulator::ops::helper::{set_flags_on_value, sign};
use crate::emulator::Cpu;
use crate::{p_get, p_set};

// http://www.6502.org/tutorials/6502opcodes.html#ASL
// http://www.6502.org/users/obelisk/6502/reference.html#ASL
pub(crate) fn asl_acc(cpu: &mut Cpu) {
    cpu.reg.a = asl_helper(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ASL
// http://www.6502.org/users/obelisk/6502/reference.html#ASL
pub(crate) fn asl(cpu: &mut Cpu, addr: u16) {
    let value = cpu.bus.load(addr);
    let value = asl_helper(cpu, value);
    cpu.bus.store(addr, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#LSR
// http://www.6502.org/users/obelisk/6502/reference.html#LSR
pub(crate) fn lsr_acc(cpu: &mut Cpu) {
    cpu.reg.a = lsr_helper(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#LSR
// http://www.6502.org/users/obelisk/6502/reference.html#LSR
pub(crate) fn lsr(cpu: &mut Cpu, addr: u16) {
    let value = cpu.bus.load(addr);
    let value = lsr_helper(cpu, value);
    cpu.bus.store(addr, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#ROL
// http://www.6502.org/users/obelisk/6502/reference.html#ROL
pub(crate) fn rol_acc(cpu: &mut Cpu) {
    cpu.reg.a = rol_helper(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ROL
// http://www.6502.org/users/obelisk/6502/reference.html#ROL
pub(crate) fn rol(cpu: &mut Cpu, addr: u16) {
    let value = cpu.bus.load(addr);
    let value = rol_helper(cpu, value);
    cpu.bus.store(addr, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#ROR
// http://www.6502.org/users/obelisk/6502/reference.html#ROR
pub(crate) fn ror_acc(cpu: &mut Cpu) {
    cpu.reg.a = ror_helper(cpu, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ROR
// http://www.6502.org/users/obelisk/6502/reference.html#ROR
pub(crate) fn ror(cpu: &mut Cpu, addr: u16) {
    let value = cpu.bus.load(addr);
    let value = ror_helper(cpu, value);
    cpu.bus.store(addr, value);
}

fn asl_helper(cpu: &mut Cpu, operand: u8) -> u8 {
    p_set!(cpu.reg, C, sign(operand));
    let new_value = operand << 1;
    set_flags_on_value(cpu, new_value);
    new_value
}

fn lsr_helper(cpu: &mut Cpu, operand: u8) -> u8 {
    p_set!(cpu.reg, C, (operand & 0x01) != 0);
    let new_value = operand >> 1;
    set_flags_on_value(cpu, new_value);
    new_value
}

fn rol_helper(cpu: &mut Cpu, operand: u8) -> u8 {
    let old_carry = p_get!(cpu.reg, C);
    p_set!(cpu.reg, C, sign(operand));
    let new_value = (operand << 1) | (if old_carry { 0x01 } else { 0x00 });
    set_flags_on_value(cpu, new_value);
    new_value
}

fn ror_helper(cpu: &mut Cpu, operand: u8) -> u8 {
    let old_carry = p_get!(cpu.reg, C);
    p_set!(cpu.reg, C, (operand & 0x01) != 0);
    let new_value = (operand >> 1) | (if old_carry { 0x80 } else { 0x00 });
    set_flags_on_value(cpu, new_value);
    new_value
}

#[cfg(test)]
mod tests {
    use crate::_p;
    use crate::emulator::ops::rol_acc;
    use crate::emulator::{Bus, Cpu};
    use rstest::rstest;

    #[rstest]
    // cargo run -- validate-json '{ "name": "2a d4 c3", "initial": { "pc": 21085, "s": 186, "a": 175, "x": 190, "y": 239, "p": 174, "ram": [ [21085, 42], [21086, 212], [21087, 195]]}, "final": { "pc": 21086, "s": 186, "a": 94, "x": 190, "y": 239, "p": 45, "ram": [ [21085, 42], [21086, 212], [21087, 195]]}, "cycles": [ [21085, 42, "read"], [21086, 212, "read"]] }'
    #[case(45, 94, 174, 175)]
    fn rol_basics(#[case] expected_p: u8, #[case] expected_a: u8, #[case] p: u8, #[case] a: u8) {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.p = _p!(p);
        cpu.reg.a = a;
        rol_acc(&mut cpu);
        assert_eq!(_p!(expected_p), cpu.reg.p);
        assert_eq!(expected_a, cpu.reg.a);
    }
}
