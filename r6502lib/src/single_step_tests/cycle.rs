use crate::single_step_tests::CycleVisitor;
use serde::{Deserialize, Deserializer};
use std::result::Result as StdResult;

#[derive(Debug)]
pub(crate) struct Cycle {
    pub(crate) address: u16,
    pub(crate) value: u8,
    pub(crate) operation: String,
}

impl<'de> Deserialize<'de> for Cycle {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(CycleVisitor)
    }
}
