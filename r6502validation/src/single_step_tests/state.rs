use crate::single_step_tests::AddressValue;
use r6502lib::P;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::result::Result as StdResult;

#[derive(Debug, Deserialize)]
pub(crate) struct State {
    #[serde(rename = "pc")]
    pub(crate) pc: u16,
    #[serde(rename = "s")]
    pub(crate) s: u8,
    #[serde(rename = "a")]
    pub(crate) a: u8,
    #[serde(rename = "x")]
    pub(crate) x: u8,
    #[serde(rename = "y")]
    pub(crate) y: u8,
    #[serde(rename = "p", deserialize_with = "deserialize_p")]
    pub(crate) p: P,
    #[serde(rename = "ram")]
    pub(crate) ram: Vec<AddressValue>,
}

fn deserialize_p<'de, D>(deserializer: D) -> StdResult<P, D::Error>
where
    D: Deserializer<'de>,
{
    let value = u8::deserialize(deserializer)?;
    P::from_bits(value).ok_or_else(|| {
        SerdeError::custom(format!(
            "Invalid value ${value:02X} ({value}) (0b{value:08b}) for P"
        ))
    })
}
