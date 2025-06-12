use anyhow::Result;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

pub fn deserialize_word<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.strip_prefix('$') {
        Some(suffix) => u16::from_str_radix(suffix, 16).map_err(SerdeError::custom),
        None => s.parse().map_err(SerdeError::custom),
    }
}
