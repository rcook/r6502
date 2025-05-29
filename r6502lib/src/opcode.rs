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
    #[strum(props(mnemonic = "BCC"))]
    Bcc = 0x90,
    #[strum(props(mnemonic = "BCS"))]
    Bcs = 0x80,
    #[strum(props(mnemonic = "BEQ"))]
    Beq = 0xf0,
    #[strum(props(mnemonic = "BMI"))]
    Bmi = 0x30,
    #[strum(props(mnemonic = "BNE"))]
    Bne = 0xd0,
    #[strum(props(mnemonic = "BPL"))]
    Bpl = 0x10,
    #[strum(props(mnemonic = "BRK"))]
    Brk = 0x00,
    #[strum(props(mnemonic = "BVC"))]
    Bvc = 0x50,
    #[strum(props(mnemonic = "BVS"))]
    Bvs = 0x70,
    #[strum(props(mnemonic = "INX"))]
    Inx = 0xe8,
    #[strum(props(mnemonic = "INY"))]
    Iny = 0xc8,
    #[strum(props(mnemonic = "JMP"))]
    JmpAbs = 0x4c,
    #[strum(props(mnemonic = "JSR"))]
    Jsr = 0x20,
    #[strum(props(mnemonic = "LDA"))]
    LdaAbs = 0xad,
    #[strum(props(mnemonic = "LDA"))]
    LdaAbsX = 0xbd,
    #[strum(props(mnemonic = "LDA"))]
    LdaImm = 0xa9,
    #[strum(props(mnemonic = "LDA"))]
    LdaZp = 0xa5,
    #[strum(props(mnemonic = "LDX"))]
    LdxAbs = 0xae,
    #[strum(props(mnemonic = "LDX"))]
    LdxImm = 0xa2,
    #[strum(props(mnemonic = "LDX"))]
    LdxZp = 0xa6,
    #[strum(props(mnemonic = "LDY"))]
    LdyAbs = 0xac,
    #[strum(props(mnemonic = "LDY"))]
    LdyImm = 0xa0,
    #[strum(props(mnemonic = "LDY"))]
    LdyZp = 0xa4,
    #[strum(props(mnemonic = "NOP"))]
    Nop = 0xea,
    #[strum(props(mnemonic = "PHA"))]
    Pha = 0x48,
    #[strum(props(mnemonic = "PHP"))]
    Php = 0x08,
    #[strum(props(mnemonic = "PLA"))]
    Pla = 0x68,
    #[strum(props(mnemonic = "PLP"))]
    Plp = 0x28,
    #[strum(props(mnemonic = "RTS"))]
    Rts = 0x60,
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
