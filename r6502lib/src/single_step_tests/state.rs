use crate::single_step_tests::AddressValue;
use serde::Deserialize;

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
    #[serde(rename = "p")]
    pub(crate) p: u8,
    #[serde(rename = "ram")]
    pub(crate) ram: Vec<AddressValue>,
}
