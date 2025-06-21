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
                match value {
                    0x0d => None, // Swallow CR
                    _ => Some(value),
                }
            }
            Self::HighBitCr => {
                match value {
                    0x7f => None,            // Filter out initialization
                    0x8d => Some(0x0a),      // Translate CR with high bit set to LF
                    _ => Some(value & 0x7f), // Clear the high bit
                }
            }
        }
    }
}
