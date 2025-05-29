use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Eq, FromPrimitive, Hash, PartialEq)]
#[repr(u8)]
pub(crate) enum Opcode {
    AdcAbs = 0x6d,
    AdcImm = 0x69,
    AdcZp = 0x65,
    JmpAbs = 0x4c,
    Nop = 0xea,
}

impl Opcode {
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        FromPrimitive::from_u8(value)
    }
}
