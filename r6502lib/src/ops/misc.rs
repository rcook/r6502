use crate::{p_set, OpCycles, VmState, IRQ};

// http://www.6502.org/tutorials/6502opcodes.html#BRK
// http://www.6502.org/users/obelisk/6502/reference.html#BRK
pub(crate) fn brk(s: &mut VmState) -> OpCycles {
    s.push_word(s.reg.pc);
    s.push(s.reg.p.bits());
    s.reg.pc = s.memory.fetch_word(IRQ);
    p_set!(s.reg, B, true);
    7
}

// http://www.6502.org/tutorials/6502opcodes.html#NOP
// http://www.6502.org/users/obelisk/6502/reference.html#NOP
pub(crate) fn nop(_s: &mut VmState) -> OpCycles {
    2
}

#[cfg(test)]
mod tests {
    use crate::ops::misc::brk;
    use crate::{p_get, reg, Memory, VmState, IRQ};

    #[test]
    fn brk_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        s.memory.store_word(IRQ, 0x1234);
        assert!(!p_get!(s.reg, B));
        assert_eq!(0x0000, s.reg.pc);
        assert_eq!(0xff, s.reg.s);
        let cycles = brk(&mut s);
        assert_eq!(7, cycles);
        assert!(p_get!(s.reg, B));
        assert_eq!(0x1234, s.reg.pc);
        assert_eq!(0xfc, s.reg.s);
    }
}

/*
pub(crate) const AND_IMM: Op = Op {
    mnemonic: "AND",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0x29,
    func: OpFunc::Byte(|m, operand| {
        m.reg.a &= operand;
        m.set_flags_for(m.reg.a);
        2
    }),
};

pub(crate) const ASL_A: Op = Op {
    mnemonic: "ASL",
    addressing_mode: AddressingMode::Accumulator,
    opcode: 0x0a,
    func: OpFunc::NoOperand(|m| {
        let value = m.reg.a;
        m.set_flag(Flag::Carry, (value & 0x80) != 0);
        m.reg.a = value << 1;
        m.set_flags_for(m.reg.a);
        2
    }),
};

pub(crate) const BIT_ABS: Op = Op {
    mnemonic: "BIT",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x2c,
    func: OpFunc::Word(|m, operand| {
        let value = m.fetch(operand);
        let result = m.reg.a & value;
        m.set_flag(Flag::Z, result == 0);
        m.set_flag(Flag::N, (value & 0x80) != 0);
        m.set_flag(Flag::V, (value & 0x40) != 0);
        4
    }),
};

pub(crate) const EOR_IMM: Op = Op {
    mnemonic: "EOR",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0x49,
    func: OpFunc::Byte(|m, operand| {
        m.reg.a ^= operand;
        m.set_flags_for(m.reg.a);
        2
    }),
};

pub(crate) const LSR_A: Op = Op {
    mnemonic: "LSR",
    addressing_mode: AddressingMode::Accumulator,
    opcode: 0x4a,
    func: OpFunc::NoOperand(|m| {
        let value = m.reg.a;
        m.set_flag(Flag::Carry, (value & 0x01) != 0);
        m.reg.a = value >> 1;
        m.set_flags_for(m.reg.a);
        2
    }),
};

pub(crate) const ORA_IMM: Op = Op {
    mnemonic: "ORA",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0x09,
    func: OpFunc::Byte(|m, operand| {
        m.reg.a |= operand;
        m.set_flags_for(m.reg.a);
        2
    }),
};

pub(crate) const ROL_A: Op = Op {
    mnemonic: "ROL",
    addressing_mode: AddressingMode::Accumulator,
    opcode: 0x2a,
    func: OpFunc::NoOperand(|m| {
        let value = m.reg.a;
        let old_carry = if m.get_flag(Flag::Carry) { 1 } else { 0 };
        let new_carry = (value & 0x80) != 0;
        m.set_flag(Flag::Carry, new_carry);
        m.reg.a = (value << 1) | old_carry;
        m.set_flags_for(m.reg.a);
        2
    }),
};

pub(crate) const ROR_A: Op = Op {
    mnemonic: "ROR",
    addressing_mode: AddressingMode::Accumulator,
    opcode: 0x6a,
    func: OpFunc::NoOperand(|m| {
        let value = m.reg.a;
        let old_carry = if m.get_flag(Flag::Carry) { 0x80 } else { 0 };
        m.set_flag(Flag::Carry, (value & 0x01) != 0);
        m.reg.a = (value >> 1) | old_carry;
        m.set_flags_for(m.reg.a);
        2
    }),
};
*/
