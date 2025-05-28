use crate::{
    compute_branch, split_word, AddressingMode, ByteFn, NoOperandFn, Op, SymbolInfo, WordFn,
};

#[derive(Clone)]
pub(crate) enum Instruction {
    NoOperand {
        #[allow(unused)]
        pc: u16,
        op: Op,
        f: NoOperandFn,
    },
    Byte {
        pc: u16,
        op: Op,
        f: ByteFn,
        operand: u8,
    },
    Word {
        #[allow(unused)]
        pc: u16,
        op: Op,
        f: WordFn,
        operand: u16,
    },
}

impl Instruction {
    pub(crate) fn pretty_current(&self, symbols: &Vec<SymbolInfo>) -> String {
        match self {
            Self::NoOperand { pc: _, op, f: _ } => {
                format!("{} ({:02X})", op.mnemonic, op.opcode)
            }
            Self::Byte {
                pc,
                op,
                f: _,
                operand,
            } => format!(
                "{} {} ({:02X} {:02X})",
                op.mnemonic,
                Self::look_up_name(symbols, *pc, op, *operand),
                op.opcode,
                operand,
            ),
            Self::Word {
                pc: _,
                op,
                f: _,
                operand,
            } => {
                let (hi, lo) = split_word(*operand);
                format!(
                    "{} {} ({:02X} {:02X} {:02X})",
                    op.mnemonic,
                    Self::look_up_name_for_word(symbols, op, *operand),
                    op.opcode,
                    lo,
                    hi,
                )
            }
        }
    }

    pub(crate) fn pretty_disassembly(&self, symbols: &Vec<SymbolInfo>) -> String {
        match self {
            Self::NoOperand { pc, op, f: _ } => {
                format!("{:04X}  {:02X}       {}", pc, op.opcode, op.mnemonic)
            }
            Self::Byte {
                pc,
                op,
                f: _,
                operand,
            } => format!(
                "{:04X}  {:02X} {:02X}    {} {}",
                pc,
                op.opcode,
                operand,
                op.mnemonic,
                Self::look_up_name(symbols, *pc, op, *operand)
            ),
            Self::Word {
                pc,
                op,
                f: _,
                operand,
            } => {
                let (hi, lo) = split_word(*operand);
                format!(
                    "{:04X}  {:02X} {:02X} {:02X} {} {}",
                    pc,
                    op.opcode,
                    lo,
                    hi,
                    op.mnemonic,
                    Self::look_up_name_for_word(symbols, op, *operand)
                )
            }
        }
    }

    fn look_up_name(symbols: &Vec<SymbolInfo>, pc: u16, op: &Op, operand: u8) -> String {
        let effective_value = if op.addressing_mode == AddressingMode::Relative {
            compute_branch(pc + 2, operand).0
        } else {
            operand as u16
        };

        for symbol in symbols {
            if symbol.value == effective_value {
                return symbol.name.clone();
            }
        }

        op.addressing_mode.pretty_byte(operand)
    }

    fn look_up_name_for_word(symbols: &Vec<SymbolInfo>, op: &Op, operand: u16) -> String {
        for symbol in symbols {
            if symbol.value == operand {
                return symbol.name.clone();
            }
        }
        op.addressing_mode.pretty_word(operand)
    }
}
