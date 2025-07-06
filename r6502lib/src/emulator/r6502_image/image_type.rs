use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum R6502ImageType {
    Module = 0,
    Snapshot = 1,
    System = 2,
}

impl R6502ImageType {
    #[must_use]
    pub const fn header_len(&self) -> u64 {
        match self {
            Self::Module => 11,
            Self::Snapshot => 22,
            Self::System => 9,
        }
    }
}
