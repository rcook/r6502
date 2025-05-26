use crate::{AddressingMode, Flag, OpFn, OSHALT, OSWRCH};

#[derive(Clone, Copy)]
pub(crate) struct Op {
    #[allow(unused)]
    pub(crate) mnemonic: &'static str,
    #[allow(unused)]
    pub(crate) addressing_mode: AddressingMode,
    pub(crate) opcode: u8,
    pub(crate) func: OpFn,
}

const BRK: Op = Op {
    mnemonic: "BRK",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x00u8,
    func: |state| {
        let pc = state.pc - 1;
        match pc {
            OSWRCH => {
                let c = state.a as char;
                state.stdout(c);
                (RTS.func)(state);
            }
            OSHALT => {
                state.running = false;
            }
            _ => panic!("Break at {:04X}", pc),
        }
    },
};

const JSR: Op = Op {
    mnemonic: "JSR",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x20u8,
    func: |state| {
        let addr = state.fetch_word();
        state.push_word(state.pc - 1);
        state.pc = addr;
    },
};

const JMP_ABS: Op = Op {
    mnemonic: "JMP",
    addressing_mode: AddressingMode::Absolute,
    opcode: 0x4cu8,
    func: |state| {
        state.pc = state.fetch_word();
    },
};

const RTS: Op = Op {
    mnemonic: "RTS",
    addressing_mode: AddressingMode::Implied,
    opcode: 0x60u8,
    func: |state| {
        state.pc = state.pull_word();
        state.pc += 1;
    },
};

const LDX_IMM: Op = Op {
    mnemonic: "LDX",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xa2u8,
    func: |state| {
        let value = state.fetch();
        state.x = value;
    },
};

const LDA_ABS_X: Op = Op {
    mnemonic: "LDA",
    addressing_mode: AddressingMode::AbsoluteX,
    opcode: 0xbdu8,
    func: |state| {
        let base_addr = state.fetch_word();
        let addr = base_addr + state.x as u16;
        let value = state.memory[addr as usize];
        state.a = value;
    },
};

const CMP_IMM: Op = Op {
    mnemonic: "CMP",
    addressing_mode: AddressingMode::Immediate,
    opcode: 0xc9u8,
    func: |state| {
        let value = state.fetch();
        let result = state.a as i32 - value as i32;
        state.set_flag(Flag::N, state.a >= 0x80u8);
        state.set_flag(Flag::Z, result == 0);
        state.set_flag(Flag::CARRY, result >= 0);
    },
};

const INX: Op = Op {
    mnemonic: "INX",
    addressing_mode: AddressingMode::Implied,
    opcode: 0xe8u8,
    func: |state| {
        state.x += 1;
    },
};

const BEQ: Op = Op {
    mnemonic: "BEQ",
    addressing_mode: AddressingMode::Relative,
    opcode: 0xf0u8,
    func: |state| {
        let value = state.fetch();
        if state.get_flag(Flag::Z) {
            match state.pc.checked_add(value as u16) {
                Some(result) => state.pc = result,
                None => todo!(),
            }
        }
    },
};

pub(crate) const OPS: [Op; 9] = [
    BRK, JSR, JMP_ABS, RTS, LDX_IMM, LDA_ABS_X, CMP_IMM, INX, BEQ,
];
