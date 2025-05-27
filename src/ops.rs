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
    func: OpFunc::NoArgs(|state| {
        state.push_word(state.pc);
        state.push(state.p);
        state.pc = state.fetch_word(IRQ);
        state.set_flag(Flag::B, true);
    }),
};

pub(crate) const JSR: Op = Op {
    mnemonic: "JSR",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x20u8,
    func: OpFunc::Word(|state, operand| {
        state.push_word(state.pc - 1);
        state.pc = operand;
    }),
};

pub(crate) const RTI: Op = Op {
    mnemonic: "RTI",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x40u8,
    func: OpFunc::NoArgs(|state| {
        state.p = state.pull();
        state.pc = state.pull_word();
    }),
};

pub(crate) const JMP_ABS: Op = Op {
    mnemonic: "JMP",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x4cu8,
    func: OpFunc::Word(|state, operand| state.pc = operand),
};

pub(crate) const RTS: Op = Op {
    mnemonic: "RTS",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x60u8,
    func: OpFunc::NoArgs(|state| {
        state.pc = state.pull_word();
        state.pc += 1;
    }),
};

pub(crate) const LDX_IMM: Op = Op {
    mnemonic: "LDX",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xa2u8,
    func: OpFunc::Byte(|state, operand| state.x = operand),
};

pub(crate) const LDA_ABS_X: Op = Op {
    mnemonic: "LDA",
    addressing_mode: AddressingMode::AbsoluteX,
    opcode: 0xbdu8,
    func: OpFunc::Word(|state, operand| {
        let addr = operand + state.x as u16;
        let value = state.memory[addr as usize];
        state.a = value;
    }),
};

pub(crate) const CMP_IMM: Op = Op {
    mnemonic: "CMP",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xc9u8,
    func: OpFunc::Byte(|state, operand| {
        let result = state.a as i32 - operand as i32;
        state.set_flag(Flag::N, state.a >= 0x80u8);
        state.set_flag(Flag::Z, result == 0);
        state.set_flag(Flag::CARRY, result >= 0);
    }),
};

pub(crate) const INX: Op = Op {
    mnemonic: "INX",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xe8u8,
    func: OpFunc::NoArgs(|state| state.x += 1),
};

pub(crate) const NOP: Op = Op {
    mnemonic: "NOP",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xeau8,
    func: OpFunc::NoArgs(|_state| {}),
};

pub(crate) const BEQ: Op = Op {
    mnemonic: "BEQ",
    addressing_mode: AddressingMode::Relative,
    opcode: 0xf0u8,
    func: OpFunc::Byte(|state, operand| {
        if state.get_flag(Flag::Z) {
            match state.pc.checked_add(operand as u16) {
                Some(result) => state.pc = result,
                None => todo!(),
            }
        }
    }),
};

pub(crate) const OPS: [Op; 11] = [
    BEQ, BRK, CMP_IMM, INX, JMP_ABS, JSR, LDA_ABS_X, LDX_IMM, NOP, RTI, RTS,
];
