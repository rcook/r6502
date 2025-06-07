use crate::ops::helper::{set_flags_on_value, sign};
use crate::{p_get, p_set, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#ASL
// http://www.6502.org/users/obelisk/6502/reference.html#ASL
pub(crate) fn asl_acc(s: &mut VmState) {
    s.reg.a = asl_helper(s, s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ASL
// http://www.6502.org/users/obelisk/6502/reference.html#ASL
pub(crate) fn asl(s: &mut VmState, addr: u16) {
    let value = s.memory.load(addr);
    let value = asl_helper(s, value);
    s.memory.store(addr, value)
}

// http://www.6502.org/tutorials/6502opcodes.html#LSR
// http://www.6502.org/users/obelisk/6502/reference.html#LSR
pub(crate) fn lsr_acc(s: &mut VmState) {
    s.reg.a = lsr_helper(s, s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#LSR
// http://www.6502.org/users/obelisk/6502/reference.html#LSR
pub(crate) fn lsr(s: &mut VmState, addr: u16) {
    let value = s.memory.load(addr);
    let value = lsr_helper(s, value);
    s.memory.store(addr, value)
}

// http://www.6502.org/tutorials/6502opcodes.html#ROL
// http://www.6502.org/users/obelisk/6502/reference.html#ROL
pub(crate) fn rol_acc(s: &mut VmState) {
    s.reg.a = rol_helper(s, s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ROL
// http://www.6502.org/users/obelisk/6502/reference.html#ROL
pub(crate) fn rol(s: &mut VmState, addr: u16) {
    let value = s.memory.load(addr);
    let value = rol_helper(s, value);
    s.memory.store(addr, value)
}

// http://www.6502.org/tutorials/6502opcodes.html#ROR
// http://www.6502.org/users/obelisk/6502/reference.html#ROR
pub(crate) fn ror_acc(s: &mut VmState) {
    s.reg.a = ror_helper(s, s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#ROR
// http://www.6502.org/users/obelisk/6502/reference.html#ROR
pub(crate) fn ror(s: &mut VmState, addr: u16) {
    let value = s.memory.load(addr);
    let value = ror_helper(s, value);
    s.memory.store(addr, value)
}

fn asl_helper(s: &mut VmState, operand: u8) -> u8 {
    p_set!(s.reg, C, sign(operand));
    let new_value = operand << 1;
    set_flags_on_value(s, new_value);
    new_value
}

fn lsr_helper(s: &mut VmState, operand: u8) -> u8 {
    p_set!(s.reg, C, (operand & 0x01) != 0);
    let new_value = operand >> 1;
    set_flags_on_value(s, new_value);
    new_value
}

fn rol_helper(s: &mut VmState, operand: u8) -> u8 {
    let old_carry = p_get!(s.reg, C);
    p_set!(s.reg, C, sign(operand));
    let new_value = (operand << 1) | (if old_carry { 0x01 } else { 0x00 });
    set_flags_on_value(s, new_value);
    new_value
}

fn ror_helper(s: &mut VmState, operand: u8) -> u8 {
    let old_carry = p_get!(s.reg, C);
    p_set!(s.reg, C, (operand & 0x01) != 0);
    let new_value = (operand >> 1) | (if old_carry { 0x80 } else { 0x00 });
    set_flags_on_value(s, new_value);
    new_value
}

#[cfg(test)]
mod tests {
    use crate::ops::rol_acc;
    use crate::{Memory, Reg, VmState, _p};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    // cargo run -p r6502validation -- run-json '{ "name": "2a d4 c3", "initial": { "pc": 21085, "s": 186, "a": 175, "x": 190, "y": 239, "p": 174, "ram": [ [21085, 42], [21086, 212], [21087, 195]]}, "final": { "pc": 21086, "s": 186, "a": 94, "x": 190, "y": 239, "p": 45, "ram": [ [21085, 42], [21086, 212], [21087, 195]]}, "cycles": [ [21085, 42, "read"], [21086, 212, "read"]] }'
    #[case(45, 94, 174, 175)]
    fn rol_basics(
        #[case] expected_p: u8,
        #[case] expected_a: u8,
        #[case] p: u8,
        #[case] a: u8,
    ) -> Result<()> {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.p = _p!(p);
        s.reg.a = a;
        rol_acc(&mut s);
        assert_eq!(_p!(expected_p), s.reg.p);
        assert_eq!(expected_a, s.reg.a);
        Ok(())
    }
}
