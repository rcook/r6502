use crate::Op;

pub(crate) use inner::*;

pub(crate) fn iter_ops() -> impl Iterator<Item = &'static Op> {
    crate::ops::CONSTS.iter().map(|(_, item)| match item {
        Item::Op(op) => op,
    })
}

#[iter_mod::make_items]
mod inner {
    use crate::{compute_branch, AddressingMode, Cycles, Flag, MachineState, Op, OpFunc, IRQ};

    pub(crate) const ADC_IMM: Op = Op {
        mnemonic: "ADC",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0x69u8,
        func: OpFunc::Byte(|m, operand| {
            let carry = if m.get_flag(Flag::Carry) { 1 } else { 0 };
            let sum = m.reg.a as u16 + operand as u16 + carry;
            let result = sum as u8;
            m.reg.a = result;
            m.set_flag(Flag::Carry, sum > 0xFF);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::N, result >= 0x80);
            m.set_flag(Flag::V, ((m.reg.a ^ result) & (operand ^ result) & 0x80) != 0);
            2
        }),
    };

    pub(crate) const ADC_ZP: Op = Op {
        mnemonic: "ADC",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x65u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            let carry = if m.get_flag(Flag::Carry) { 1 } else { 0 };
            let sum = m.reg.a as u16 + value as u16 + carry;
            let result = sum as u8;
            m.reg.a = result;
            m.set_flag(Flag::Carry, sum > 0xFF);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::N, result >= 0x80);
            m.set_flag(Flag::V, ((m.reg.a ^ result) & (value ^ result) & 0x80) != 0);
            3
        }),
    };

    pub(crate) const AND_IMM: Op = Op {
        mnemonic: "AND",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0x29u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.a &= operand;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const ASL_A: Op = Op {
        mnemonic: "ASL",
        addressing_mode: AddressingMode::Accumulator,
        opcode: 0x0Au8,
        func: OpFunc::NoOperand(|m| {
            let value = m.reg.a;
            m.set_flag(Flag::Carry, (value & 0x80) != 0);
            m.reg.a = value << 1;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const ASL_ZP: Op = Op {
        mnemonic: "ASL",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x06u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            m.set_flag(Flag::Carry, (value & 0x80) != 0);
            let result = value << 1;
            m.store(operand as u16, result);
            m.set_flags_for(result);
            5
        }),
    };

    pub(crate) const BCC: Op = Op {
        mnemonic: "BCC",
        addressing_mode: AddressingMode::Relative,
        opcode: 0x90u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::Carry, false, operand)),
    };

    pub(crate) const BCS: Op = Op {
        mnemonic: "BCS",
        addressing_mode: AddressingMode::Relative,
        opcode: 0xB0u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::Carry, true, operand)),
    };

    pub(crate) const BEQ: Op = Op {
        mnemonic: "BEQ",
        addressing_mode: AddressingMode::Relative,
        opcode: 0xf0u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::Z, true, operand)),
    };

    pub(crate) const BIT_ABS: Op = Op {
        mnemonic: "BIT",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0x2Cu8,
        func: OpFunc::Word(|m, operand| {
            let value = m.fetch(operand);
            let result = m.reg.a & value;
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::N, (value & 0x80) != 0);
            m.set_flag(Flag::V, (value & 0x40) != 0);
            4
        }),
    };

    pub(crate) const BIT_ZP: Op = Op {
        mnemonic: "BIT",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x24u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            let result = m.reg.a & value;
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::N, (value & 0x80) != 0);
            m.set_flag(Flag::V, (value & 0x40) != 0);
            3
        }),
    };

    pub(crate) const BMI: Op = Op {
        mnemonic: "BMI",
        addressing_mode: AddressingMode::Relative,
        opcode: 0x30u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::N, true, operand)),
    };

    pub(crate) const BNE: Op = Op {
        mnemonic: "BNE",
        addressing_mode: AddressingMode::Relative,
        opcode: 0xd0u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::Z, false, operand)),
    };

    pub(crate) const BPL: Op = Op {
        mnemonic: "BPL",
        addressing_mode: AddressingMode::Relative,
        opcode: 0x10u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::N, false, operand)),
    };

    pub(crate) const BRK: Op = Op {
        mnemonic: "BRK",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x00u8,
        func: OpFunc::NoOperand(|m| {
            m.push_word(m.reg.pc);
            m.push(m.reg.p);
            m.reg.pc = m.fetch_word(IRQ);
            m.set_flag(Flag::B, true);
            7
        }),
    };

    pub(crate) const BVC: Op = Op {
        mnemonic: "BVC",
        addressing_mode: AddressingMode::Relative,
        opcode: 0x50u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::V, false, operand)),
    };

    pub(crate) const BVS: Op = Op {
        mnemonic: "BVS",
        addressing_mode: AddressingMode::Relative,
        opcode: 0x70u8,
        func: OpFunc::Byte(|m, operand| branch(m, Flag::V, true, operand)),
    };

    pub(crate) const CLC: Op = Op {
        mnemonic: "CLC",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x18u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::Carry, false);
            2
        }),
    };

    pub(crate) const CLD: Op = Op {
        mnemonic: "CLD",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xD8u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::D, false);
            2
        }),
    };

    pub(crate) const CLI: Op = Op {
        mnemonic: "CLI",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x58u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::I, false);
            2
        }),
    };

    pub(crate) const CLV: Op = Op {
        mnemonic: "CLV",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xB8u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::V, false);
            2
        }),
    };

    pub(crate) const CMP_IMM: Op = Op {
        mnemonic: "CMP",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xc9u8,
        func: OpFunc::Byte(|m, operand| {
            let result = m.reg.a as i32 - operand as i32;
            m.set_flag(Flag::N, m.reg.a >= 0x80u8);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            2
        }),
    };

    pub(crate) const CPX_ABS: Op = Op {
        mnemonic: "CPX",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0xECu8,
        func: OpFunc::Word(|m, operand| {
            let value = m.fetch(operand);
            let result = m.reg.x as i32 - value as i32;
            m.set_flag(Flag::N, result < 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            4
        }),
    };

    pub(crate) const CPX_IMM: Op = Op {
        mnemonic: "CPX",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xe0u8,
        func: OpFunc::Byte(|m, operand| {
            let result = m.reg.x as i32 - operand as i32;
            m.set_flag(Flag::N, result < 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            2
        }),
    };

    pub(crate) const CPX_ZP: Op = Op {
        mnemonic: "CPX",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0xE4u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            let result = m.reg.x as i32 - value as i32;
            m.set_flag(Flag::N, result < 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            3
        }),
    };

    pub(crate) const CPY_ABS: Op = Op {
        mnemonic: "CPY",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0xCCu8,
        func: OpFunc::Word(|m, operand| {
            let value = m.fetch(operand);
            let result = m.reg.y as i32 - value as i32;
            m.set_flag(Flag::N, result < 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            4
        }),
    };

    pub(crate) const CPY_IMM: Op = Op {
        mnemonic: "CPY",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xC0u8,
        func: OpFunc::Byte(|m, operand| {
            let result = m.reg.y as i32 - operand as i32;
            m.set_flag(Flag::N, result < 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            2
        }),
    };

    pub(crate) const CPY_ZP: Op = Op {
        mnemonic: "CPY",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0xC4u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            let result = m.reg.y as i32 - value as i32;
            m.set_flag(Flag::N, result < 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::Carry, result >= 0);
            3
        }),
    };

    pub(crate) const DEX: Op = Op {
        mnemonic: "DEX",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xcau8,
        func: OpFunc::NoOperand(|m| {
            let operand = m.reg.x - 1;
            m.reg.x = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const EOR_IMM: Op = Op {
        mnemonic: "EOR",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0x49u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.a ^= operand;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const INX: Op = Op {
        mnemonic: "INX",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xe8u8,
        func: OpFunc::NoOperand(|m| {
            let operand = m.reg.x + 1;
            m.reg.x = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const INY: Op = Op {
        mnemonic: "INY",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xc8u8,
        func: OpFunc::NoOperand(|m| {
            let operand = m.reg.y + 1;
            m.reg.y = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const JMP_ABS: Op = Op {
        mnemonic: "JMP",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0x4cu8,
        func: OpFunc::Word(|m, operand| {
            m.reg.pc = operand;
            3
        }),
    };

    pub(crate) const JSR: Op = Op {
        mnemonic: "JSR",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0x20u8,
        func: OpFunc::Word(|m, operand| {
            m.push_word(m.reg.pc - 1);
            m.reg.pc = operand;
            6
        }),
    };

    pub(crate) const LDA_ABS_X: Op = Op {
        mnemonic: "LDA",
        addressing_mode: AddressingMode::AbsoluteX,
        opcode: 0xbdu8,
        func: OpFunc::Word(|m, operand| {
            let addr = operand + m.reg.x as u16;
            let value = m.memory[addr as usize];
            m.reg.a = value;
            m.set_flags_for(value);
            4 // TBD: Add 1 cycle if page boundary crossed
        }),
    };

    pub(crate) const LDA_IMM: Op = Op {
        mnemonic: "LDA",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xa9u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.a = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const LDA_IND_IDX_Y: Op = Op {
        mnemonic: "LDA",
        addressing_mode: AddressingMode::IndirectIndexedY,
        opcode: 0xb1u8,
        func: OpFunc::Byte(|m, operand| {
            let base_addr = m.fetch_word(operand as u16);
            let addr = base_addr + m.reg.y as u16;
            let value = m.fetch(addr);
            m.reg.a = value;
            m.set_flags_for(value);
            5 // TBD: Add 1 cycle if page boundary crossed
        }),
    };

    pub(crate) const LDA_ZP: Op = Op {
        mnemonic: "LDA",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0xa5u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            m.reg.a = value;
            m.set_flags_for(value);
            3
        }),
    };

    pub(crate) const LDX_ABS: Op = Op {
        mnemonic: "LDX",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0xAEu8,
        func: OpFunc::Word(|m, operand| {
            m.reg.x = m.fetch(operand);
            m.set_flags_for(m.reg.x);
            4
        }),
    };

    pub(crate) const LDX_IMM: Op = Op {
        mnemonic: "LDX",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xa2u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.x = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const LDX_ZP: Op = Op {
        mnemonic: "LDX",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0xA6u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.x = m.fetch(operand as u16);
            m.set_flags_for(m.reg.x);
            3
        }),
    };

    pub(crate) const LDY_ABS: Op = Op {
        mnemonic: "LDY",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0xACu8,
        func: OpFunc::Word(|m, operand| {
            m.reg.y = m.fetch(operand);
            m.set_flags_for(m.reg.y);
            4
        }),
    };

    pub(crate) const LDY_IMM: Op = Op {
        mnemonic: "LDY",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xa0u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.y = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const LDY_ZP: Op = Op {
        mnemonic: "LDY",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0xA4u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.y = m.fetch(operand as u16);
            m.set_flags_for(m.reg.y);
            3
        }),
    };

    pub(crate) const LSR_A: Op = Op {
        mnemonic: "LSR",
        addressing_mode: AddressingMode::Accumulator,
        opcode: 0x4Au8,
        func: OpFunc::NoOperand(|m| {
            let value = m.reg.a;
            m.set_flag(Flag::Carry, (value & 0x01) != 0);
            m.reg.a = value >> 1;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const LSR_ZP: Op = Op {
        mnemonic: "LSR",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x46u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            m.set_flag(Flag::Carry, (value & 0x01) != 0);
            let result = value >> 1;
            m.store(operand as u16, result);
            m.set_flags_for(result);
            5
        }),
    };

    pub(crate) const NOP: Op = Op {
        mnemonic: "NOP",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xeau8,
        func: OpFunc::NoOperand(|_| 2),
    };

    pub(crate) const ORA_IMM: Op = Op {
        mnemonic: "ORA",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0x09u8,
        func: OpFunc::Byte(|m, operand| {
            m.reg.a |= operand;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const PHA: Op = Op {
        mnemonic: "PHA",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x48u8,
        func: OpFunc::NoOperand(|m| {
            m.push(m.reg.a);
            3
        }),
    };

    pub(crate) const PHP: Op = Op {
        mnemonic: "PHP",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x08u8,
        func: OpFunc::NoOperand(|m| {
            m.push(m.reg.p | 0x10); // Set B flag when pushing
            3
        }),
    };

    pub(crate) const PLA: Op = Op {
        mnemonic: "PLA",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x68u8,
        func: OpFunc::NoOperand(|m| {
            m.reg.a = m.pull();
            4
        }),
    };

    pub(crate) const PLP: Op = Op {
        mnemonic: "PLP",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x28u8,
        func: OpFunc::NoOperand(|m| {
            m.reg.p = m.pull() & 0xEF; // Clear B flag when pulling
            4
        }),
    };

    pub(crate) const ROL_A: Op = Op {
        mnemonic: "ROL",
        addressing_mode: AddressingMode::Accumulator,
        opcode: 0x2Au8,
        func: OpFunc::NoOperand(|m| {
            let value = m.reg.a;
            let carry = if m.get_flag(Flag::Carry) { 1 } else { 0 };
            m.set_flag(Flag::Carry, (value & 0x80) != 0);
            m.reg.a = (value << 1) | carry;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const ROL_ZP: Op = Op {
        mnemonic: "ROL",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x26u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            let carry = if m.get_flag(Flag::Carry) { 1 } else { 0 };
            m.set_flag(Flag::Carry, (value & 0x80) != 0);
            let result = (value << 1) | carry;
            m.store(operand as u16, result);
            m.set_flags_for(result);
            5
        }),
    };

    pub(crate) const ROR_A: Op = Op {
        mnemonic: "ROR",
        addressing_mode: AddressingMode::Accumulator,
        opcode: 0x6Au8,
        func: OpFunc::NoOperand(|m| {
            let value = m.reg.a;
            let carry = if m.get_flag(Flag::Carry) { 0x80 } else { 0 };
            m.set_flag(Flag::Carry, (value & 0x01) != 0);
            m.reg.a = (value >> 1) | carry;
            m.set_flags_for(m.reg.a);
            2
        }),
    };

    pub(crate) const ROR_ZP: Op = Op {
        mnemonic: "ROR",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x66u8,
        func: OpFunc::Byte(|m, operand| {
            let value = m.fetch(operand as u16);
            let carry = if m.get_flag(Flag::Carry) { 0x80 } else { 0 };
            m.set_flag(Flag::Carry, (value & 0x01) != 0);
            let result = (value >> 1) | carry;
            m.store(operand as u16, result);
            m.set_flags_for(result);
            5
        }),
    };

    pub(crate) const RTI: Op = Op {
        mnemonic: "RTI",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x40u8,
        func: OpFunc::NoOperand(|m| {
            m.reg.p = m.pull();
            m.reg.pc = m.pull_word();
            6
        }),
    };

    pub(crate) const RTS: Op = Op {
        mnemonic: "RTS",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x60u8,
        func: OpFunc::NoOperand(|m| {
            m.reg.pc = m.pull_word();
            m.reg.pc += 1;
            6
        }),
    };

    pub(crate) const SBC_IMM: Op = Op {
        mnemonic: "SBC",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xE9u8,
        func: OpFunc::Byte(|m, operand| {
            let carry = if m.get_flag(Flag::Carry) { 1 } else { 0 };
            let diff = m.reg.a as i16 - operand as i16 - (1 - carry);
            let result = diff as u8;
            m.reg.a = result;
            m.set_flag(Flag::Carry, diff >= 0);
            m.set_flag(Flag::Z, result == 0);
            m.set_flag(Flag::N, result >= 0x80);
            m.set_flag(Flag::V, ((m.reg.a ^ result) & (operand ^ result) & 0x80) != 0);
            2
        }),
    };

    pub(crate) const SEC: Op = Op {
        mnemonic: "SEC",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x38u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::Carry, true);
            2
        }),
    };

    pub(crate) const SED: Op = Op {
        mnemonic: "SED",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xF8u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::D, true);
            2
        }),
    };

    pub(crate) const SEI: Op = Op {
        mnemonic: "SEI",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x78u8,
        func: OpFunc::NoOperand(|m| {
            m.set_flag(Flag::I, true);
            2
        }),
    };

    pub(crate) const STA_ABS: Op = Op {
        mnemonic: "STA",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0x8du8,
        func: OpFunc::Word(|m, operand| {
            m.store(operand, m.reg.a);
            4
        }),
    };

    pub(crate) const STA_ABS_X: Op = Op {
        mnemonic: "STA",
        addressing_mode: AddressingMode::AbsoluteX,
        opcode: 0x9du8,
        func: OpFunc::Word(|m, operand| {
            let addr = operand + m.reg.x as u16;
            m.store(addr, m.reg.a);
            5 // TBD: Add 1 cycle if page boundary crossed
        }),
    };

    pub(crate) const STA_ZP: Op = Op {
        mnemonic: "STA",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x85u8,
        func: OpFunc::Byte(|m, operand| {
            m.store(operand as u16, m.reg.a);
            3
        }),
    };

    pub(crate) const STX_ABS: Op = Op {
        mnemonic: "STX",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0x8Eu8,
        func: OpFunc::Word(|m, operand| {
            m.store(operand, m.reg.x);
            4
        }),
    };

    pub(crate) const STX_ZP: Op = Op {
        mnemonic: "STX",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x86u8,
        func: OpFunc::Byte(|m, operand| {
            m.store(operand as u16, m.reg.x);
            3
        }),
    };

    pub(crate) const STY_ABS: Op = Op {
        mnemonic: "STY",
        addressing_mode: AddressingMode::Absolute,
        opcode: 0x8Cu8,
        func: OpFunc::Word(|m, operand| {
            m.store(operand, m.reg.y);
            4
        }),
    };

    pub(crate) const STY_ZP: Op = Op {
        mnemonic: "STY",
        addressing_mode: AddressingMode::ZeroPage,
        opcode: 0x84u8,
        func: OpFunc::Byte(|m, operand| {
            m.store(operand as u16, m.reg.y);
            3
        }),
    };

    pub(crate) const TAX: Op = Op {
        mnemonic: "TAX",
        addressing_mode: AddressingMode::Implied,
        opcode: 0xaau8,
        func: OpFunc::NoOperand(|m| {
            let operand = m.reg.a;
            m.reg.x = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const TAY: Op = Op {
        mnemonic: "TAY",
        addressing_mode: AddressingMode::Immediate,
        opcode: 0xa8u8,
        func: OpFunc::NoOperand(|m| {
            let operand = m.reg.a;
            m.reg.y = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    pub(crate) const TYA: Op = Op {
        mnemonic: "TYA",
        addressing_mode: AddressingMode::Implied,
        opcode: 0x98u8,
        func: OpFunc::NoOperand(|m| {
            let operand = m.reg.y;
            m.reg.a = operand;
            m.set_flags_for(operand);
            2
        }),
    };

    fn branch(m: &mut MachineState, flag: Flag, branch_on: bool, operand: u8) -> Cycles {
        if m.get_flag(flag) == branch_on {
            let result = compute_branch(m.reg.pc, operand);
            m.reg.pc = result.0;
            result.1
        } else {
            2
        }
    }
}
