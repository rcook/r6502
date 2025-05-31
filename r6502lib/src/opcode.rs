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
    AdcAbsX = 0x7d,
    #[strum(props(mnemonic = "ADC"))]
    AdcAbsY = 0x79,
    #[strum(props(mnemonic = "ADC"))]
    AdcImm = 0x69,
    #[strum(props(mnemonic = "ADC"))]
    AdcIndX = 0x61,
    #[strum(props(mnemonic = "ADC"))]
    AdcIndY = 0x71,
    #[strum(props(mnemonic = "ADC"))]
    AdcZp = 0x65,
    #[strum(props(mnemonic = "ADC"))]
    AdcZpX = 0x75,
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
    #[strum(props(mnemonic = "CLC"))]
    Clc = 0x18,
    #[strum(props(mnemonic = "Cld"))]
    Cld = 0xd8,
    #[strum(props(mnemonic = "CLI"))]
    Cli = 0x58,
    #[strum(props(mnemonic = "Clv"))]
    Clv = 0xb8,
    #[strum(props(mnemonic = "CMP"))]
    CmpAbs = 0xcd,
    #[strum(props(mnemonic = "CMP"))]
    CmpAbsX = 0xdd,
    #[strum(props(mnemonic = "CMP"))]
    CmpImm = 0xc9,
    #[strum(props(mnemonic = "CMP"))]
    CmpZp = 0xc5,
    #[strum(props(mnemonic = "CPX"))]
    CpxAbs = 0xec,
    #[strum(props(mnemonic = "CPX"))]
    CpxImm = 0xe0,
    #[strum(props(mnemonic = "CPX"))]
    CpxZp = 0xe4,
    #[strum(props(mnemonic = "CPY"))]
    CpyAbs = 0xcc,
    #[strum(props(mnemonic = "CPY"))]
    CpyImm = 0xc0,
    #[strum(props(mnemonic = "CPY"))]
    CpyZp = 0xc4,
    #[strum(props(mnemonic = "DEX"))]
    Dex = 0xca,
    #[strum(props(mnemonic = "DEY"))]
    Dey = 0x88,
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
    LdaIndY = 0xb1,
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
    #[strum(props(mnemonic = "SEC"))]
    Sec = 0x38,
    #[strum(props(mnemonic = "Sed"))]
    Sed = 0xf8,
    #[strum(props(mnemonic = "Sei"))]
    Sei = 0x78,
    #[strum(props(mnemonic = "STA"))]
    StaAbs = 0x8d,
    #[strum(props(mnemonic = "STA"))]
    StaAbsX = 0x9d,
    #[strum(props(mnemonic = "STA"))]
    StaAbsY = 0x99,
    #[strum(props(mnemonic = "STA"))]
    StaZp = 0x85,
    #[strum(props(mnemonic = "TAX"))]
    Tax = 0xaa,
    #[strum(props(mnemonic = "TAY"))]
    Tay = 0xa8,
    #[strum(props(mnemonic = "TXA"))]
    Txa = 0x8a,
    #[strum(props(mnemonic = "TYA"))]
    Tya = 0x98,
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
