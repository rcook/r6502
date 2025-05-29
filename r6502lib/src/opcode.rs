use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::{Display, Formatter, Result as FmtResult};
use strum::EnumProperty;
use strum_macros::EnumProperty;

#[derive(Clone, Copy, Debug, EnumProperty, Eq, FromPrimitive, Hash, PartialEq)]
#[repr(u8)]
pub(crate) enum Opcode {
    #[strum(props(mnemonic = "ADC"))]
    AdcAbs = 0x6d,
    #[strum(props(mnemonic = "ADC"))]
    AdcImm = 0x69,
    #[strum(props(mnemonic = "ADC"))]
    AdcZp = 0x65,
    #[strum(props(mnemonic = "BRK"))]
    Brk = 0x00,
    #[strum(props(mnemonic = "JMP"))]
    JmpAbs = 0x4c,
    #[strum(props(mnemonic = "NOP"))]
    Nop = 0xea,
    #[strum(props(mnemonic = "PHA"))]
    Pha = 0x48,
}

impl Opcode {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        FromPrimitive::from_u8(value)
    }

    pub(crate) fn mnemonic(&self) -> &'static str {
        match self.get_str("mnemonic") {
            Some(s) => s,
            None => panic!("mnemonic must be defined for opcode {self}"),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "${:02X}", *self as u8)
    }
}
