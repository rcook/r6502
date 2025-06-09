use crate::CycleVisitor;
use serde::{Deserialize, Deserializer};
use std::result::Result as StdResult;

#[derive(Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Cycle {
    pub address: u16,
    pub value: u8,
    pub operation: String,
}

impl<'de> Deserialize<'de> for Cycle {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CycleVisitor)
    }
}
