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
    pub(crate) fn pretty_byte(&self, operand: u8, name: &Option<String>) -> String {
        let s = name.clone().unwrap_or_else(|| format!("${operand:02X}"));
        match self {
            Self::Accumulator => String::from("A"),
            Self::Immediate => format!("#{s}"),
            Self::IndexedIndirectX => format!("({s},X)"),
            Self::IndirectIndexedY => format!("({s}),Y"),
            Self::Relative => s,
            Self::ZeroPage => s,
            Self::ZeroPageX => format!("{s},X"),
            Self::ZeroPageY => format!("{s},Y"),
            _ => unimplemented!(),
        }
    }

    pub(crate) fn pretty_word(&self, operand: u16, name: &Option<String>) -> String {
        let s = name.clone().unwrap_or_else(|| format!("${operand:04X}"));
        match self {
            Self::Absolute => s,
            Self::AbsoluteX => format!("{s},X"),
            Self::AbsoluteY => format!("{s},Y"),
            _ => unimplemented!(),
        }
    }
}
