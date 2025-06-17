use strum_macros::EnumString;

#[derive(Debug, EnumString, PartialEq)]
pub enum ExportKind {
    #[strum(serialize = "EA")]
    Ea,

    #[strum(serialize = "LA")]
    La,

    #[strum(serialize = "LZ")]
    Lz,

    #[strum(serialize = "REA")]
    Rea,

    #[strum(serialize = "RLA")]
    Rla,

    #[strum(serialize = "RLZ")]
    Rlz,
}
