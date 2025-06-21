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
    pub fn translate(&self, value: u8) -> Option<u8> {
        match self {
            Self::Crlf => {
                if value == 0x0d {
                    // Swallow CR
                    None
                } else {
                    Some(value)
                }
            }
            Self::HighBitCr => {
                if value == 0x8d {
                    // Translate CR with high bit set to LF
                    Some(0x0a)
                } else {
                    // Clear the high bit
                    Some(value & 0x7f)
                }
            }
        }
    }
}
