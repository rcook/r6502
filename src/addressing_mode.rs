#[derive(Clone, Copy, Debug)]
pub(crate) enum AddressingMode {
    Absolute,
    AbsoluteX,
    Immediate,
    Implied,
    IndirectIndexedY,
    Relative,
    ZeroPage,
}

impl AddressingMode {
    pub(crate) fn pretty_byte(&self, operand: u8) -> String {
        match self {
            Self::Immediate => format!("#${operand:02X}"),
            Self::IndirectIndexedY => format!("(${operand:02X}),Y"),
            Self::Relative => format!("${operand:02X}"),
            Self::ZeroPage => format!("${operand:02X}"),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn pretty_word(&self, operand: u16) -> String {
        match self {
            Self::Absolute => format!("${operand:04X}"),
            Self::AbsoluteX => format!("${operand:04X},X"),
            _ => unimplemented!(),
        }
    }
}
