use anyhow::Result;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::result::Result as StdResult;

fn parse_word(s: &str) -> Result<u16> {
    Ok(match s.strip_prefix('$') {
        Some(suffix) => u16::from_str_radix(suffix, 16)?,
        None => s.parse::<u16>()?,
    })
}

pub fn deserialize_word<'de, D>(deserializer: D) -> StdResult<u16, D::Error>
where
    D: Deserializer<'de>,
{
    parse_word(&String::deserialize(deserializer)?).map_err(SerdeError::custom)
}

pub fn deserialize_word_opt<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(s) => Ok(Some(parse_word(&s).map_err(SerdeError::custom)?)),
        None => Ok(None),
    }
}
