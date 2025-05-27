use crate::{AddressingMode, Flag, Op, OpFunc, IRQ};

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

pub(crate) const PHA: Op = Op {
    mnemonic: "PHA",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x48u8,
    func: OpFunc::NoOperand(|m| {
        m.push(m.reg.a);
        3
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

pub(crate) const PLA: Op = Op {
    mnemonic: "PLA",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x68u8,
    func: OpFunc::NoOperand(|m| {
        m.reg.a = m.pull();
        4
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

pub(crate) const STA_ABS: Op = Op {
    mnemonic: "STA",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x8du8,
    func: OpFunc::Word(|m, operand| {
        m.store(operand, m.reg.a);
        4
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

pub(crate) const CPX_IMM: Op = Op {
    mnemonic: "CPX",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xe0u8,
    func: OpFunc::Byte(|m, operand| {
        let result = m.reg.x as i32 - operand as i32;
        m.set_flag(Flag::N, m.reg.x >= 0x80u8);
        m.set_flag(Flag::Z, result == 0);
        m.set_flag(Flag::Carry, result >= 0);
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

pub(crate) const NOP: Op = Op {
    mnemonic: "NOP",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xeau8,
    func: OpFunc::NoOperand(|_| 2),
};

pub(crate) const BEQ: Op = Op {
    mnemonic: "BEQ",
    addressing_mode: AddressingMode::Relative,
    opcode: 0xf0u8,
    func: OpFunc::Byte(|m, operand| {
        if m.get_flag(Flag::Z) {
            match m.reg.pc.checked_add(operand as u16) {
                Some(result) => {
                    m.reg.pc = result;
                    3 // TBD: Add 1 cycle if page boundary crossed
                }
                None => todo!(),
            }
        } else {
            2
        }
    }),
};

pub(crate) const OPS: [Op; 24] = [
    BEQ,
    BRK,
    CMP_IMM,
    CPX_IMM,
    DEX,
    INX,
    INY,
    JMP_ABS,
    JSR,
    LDA_ABS_X,
    LDA_IMM,
    LDA_IND_IDX_Y,
    LDX_IMM,
    LDY_IMM,
    NOP,
    PHA,
    PLA,
    RTI,
    RTS,
    STA_ABS,
    STA_ZP,
    TAX,
    TAY,
    TYA,
];
