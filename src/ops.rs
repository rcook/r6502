use crate::{AddressingMode, Flag, OpFunc, IRQ};

#[derive(Clone, Copy)]
pub(crate) struct Op {
    #[allow(unused)]
    pub(crate) mnemonic: &'static str,
    #[allow(unused)]
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) opcode: u8,
    pub(crate) func: OpFunc,
}

pub(crate) const BRK: Op = Op {
    mnemonic: "BRK",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x00u8,
    func: OpFunc::NoOperand(|cpu| {
        cpu.push_word(cpu.pc);
        cpu.push(cpu.p);
        cpu.pc = cpu.fetch_word(IRQ);
        cpu.set_flag(Flag::B, true);
    }),
};

pub(crate) const JSR: Op = Op {
    mnemonic: "JSR",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x20u8,
    func: OpFunc::Word(|cpu, operand| {
        cpu.push_word(cpu.pc - 1);
        cpu.pc = operand;
    }),
};

pub(crate) const RTI: Op = Op {
    mnemonic: "RTI",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x40u8,
    func: OpFunc::NoOperand(|cpu| {
        cpu.p = cpu.pull();
        cpu.pc = cpu.pull_word();
    }),
};

pub(crate) const PHA: Op = Op {
    mnemonic: "PHA",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x48u8,
    func: OpFunc::NoOperand(|cpu| cpu.push(cpu.a)),
};

pub(crate) const JMP_ABS: Op = Op {
    mnemonic: "JMP",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x4cu8,
    func: OpFunc::Word(|cpu, operand| cpu.pc = operand),
};

pub(crate) const RTS: Op = Op {
    mnemonic: "RTS",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x60u8,
    func: OpFunc::NoOperand(|cpu| {
        cpu.pc = cpu.pull_word();
        cpu.pc += 1;
    }),
};

pub(crate) const PLA: Op = Op {
    mnemonic: "PLA",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x68u8,
    func: OpFunc::NoOperand(|cpu| cpu.a = cpu.pull()),
};

pub(crate) const STA_ZP: Op = Op {
    mnemonic: "STA",
    addressing_mode: AddressingMode::ZeroPage,
    opcode: 0x85u8,
    func: OpFunc::Byte(|cpu, operand| cpu.store(operand as u16, cpu.a)),
};

pub(crate) const STA_ABS: Op = Op {
    mnemonic: "STA",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x8du8,
    func: OpFunc::Word(|cpu, operand| cpu.store(operand, cpu.a)),
};

pub(crate) const TYA: Op = Op {
    mnemonic: "TYA",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x98u8,
    func: OpFunc::NoOperand(|cpu| cpu.a = cpu.y),
};

pub(crate) const LDY_IMM: Op = Op {
    mnemonic: "LDY",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xa0u8,
    func: OpFunc::Byte(|cpu, operand| cpu.y = operand),
};

pub(crate) const LDX_IMM: Op = Op {
    mnemonic: "LDX",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xa2u8,
    func: OpFunc::Byte(|cpu, operand| cpu.x = operand),
};

pub(crate) const TAY: Op = Op {
    mnemonic: "TAY",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xa8u8,
    func: OpFunc::NoOperand(|cpu| cpu.y = cpu.a),
};

pub(crate) const LDA_IMM: Op = Op {
    mnemonic: "LDA",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xa9u8,
    func: OpFunc::Byte(|cpu, operand| cpu.a = operand),
};

pub(crate) const TAX: Op = Op {
    mnemonic: "TAX",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xaau8,
    func: OpFunc::NoOperand(|cpu| cpu.x = cpu.a),
};

pub(crate) const LDA_IND_IDX_Y: Op = Op {
    mnemonic: "LDA",
    addressing_mode: AddressingMode::IndirectIndexedY,
    opcode: 0xb1u8,
    func: OpFunc::Byte(|cpu, operand| {
        let base_addr = cpu.fetch_word(operand as u16);
        let addr = base_addr + cpu.y as u16;
        cpu.a = cpu.fetch(addr)
    }),
};

pub(crate) const LDA_ABS_X: Op = Op {
    mnemonic: "LDA",
    addressing_mode: AddressingMode::AbsoluteX,
    opcode: 0xbdu8,
    func: OpFunc::Word(|cpu, operand| {
        let addr = operand + cpu.x as u16;
        let value = cpu.memory[addr as usize];
        cpu.a = value;
    }),
};

pub(crate) const INY: Op = Op {
    mnemonic: "INY",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xc8u8,
    func: OpFunc::NoOperand(|cpu| cpu.y += 1),
};

pub(crate) const CMP_IMM: Op = Op {
    mnemonic: "CMP",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xc9u8,
    func: OpFunc::Byte(|cpu, operand| {
        let result = cpu.a as i32 - operand as i32;
        cpu.set_flag(Flag::N, cpu.a >= 0x80u8);
        cpu.set_flag(Flag::Z, result == 0);
        cpu.set_flag(Flag::CARRY, result >= 0);
    }),
};

pub(crate) const DEX: Op = Op {
    mnemonic: "DEX",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xcau8,
    func: OpFunc::NoOperand(|cpu| cpu.x -= 1),
};

pub(crate) const CPX_IMM: Op = Op {
    mnemonic: "CPX",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xe0u8,
    func: OpFunc::Byte(|cpu, operand| {
        let result = cpu.x as i32 - operand as i32;
        cpu.set_flag(Flag::N, cpu.x >= 0x80u8);
        cpu.set_flag(Flag::Z, result == 0);
        cpu.set_flag(Flag::CARRY, result >= 0);
    }),
};

pub(crate) const INX: Op = Op {
    mnemonic: "INX",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xe8u8,
    func: OpFunc::NoOperand(|cpu| cpu.x += 1),
};

pub(crate) const NOP: Op = Op {
    mnemonic: "NOP",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xeau8,
    func: OpFunc::NoOperand(|_cpu| {}),
};

pub(crate) const BEQ: Op = Op {
    mnemonic: "BEQ",
    addressing_mode: AddressingMode::Relative,
    opcode: 0xf0u8,
    func: OpFunc::Byte(|cpu, operand| {
        if cpu.get_flag(Flag::Z) {
            match cpu.pc.checked_add(operand as u16) {
                Some(result) => cpu.pc = result,
                None => todo!(),
            }
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
