#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    IndexedIndirectX,
    IndirectIndexedY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

impl AddressingMode {
    pub(crate) fn pretty_byte(&self, operand: u8) -> String {
        match self {
            Self::Accumulator => "A".to_string(),
            Self::Immediate => format!("#${operand:02X}"),
            Self::IndexedIndirectX => format!("(${operand:02X},X)"),
            Self::IndirectIndexedY => format!("(${operand:02X}),Y"),
            Self::Relative => format!("${operand:02X}"),
            Self::ZeroPage => format!("${operand:02X}"),
            Self::ZeroPageX => format!("${operand:02X},X"),
            Self::ZeroPageY => format!("${operand:02X},Y"),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn pretty_word(&self, operand: u16) -> String {
        match self {
            Self::Absolute => format!("${operand:04X}"),
            Self::AbsoluteX => format!("${operand:04X},X"),
            Self::AbsoluteY => format!("${operand:04X},Y"),
            _ => unimplemented!(),
        }
    }
}
