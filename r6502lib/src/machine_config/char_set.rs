use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum CharSet {
    #[serde(rename = "crlf")]
    Crlf,

    #[serde(rename = "highbitcr")]
    HighBitCr,
}

impl CharSet {
    #[must_use]
    pub fn translate(&self, value: u8) -> u8 {
        match self {
            Self::Crlf => todo!(),
            Self::HighBitCr => {
                if value == 0x8d {
                    // Translate CR with high bit set to LF
                    0x0a
                } else {
                    // Clear the high bit
                    value & 0x7f
                }
            }
        }
    }
}
