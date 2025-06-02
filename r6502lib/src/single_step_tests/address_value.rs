use crate::single_step_tests::AddressValueVisitor;
use serde::{Deserialize, Deserializer};
use std::result::Result as StdResult;

#[derive(Debug)]
pub(crate) struct AddressValue {
    pub(crate) address: u16,
    pub(crate) value: u8,
}

impl<'de> Deserialize<'de> for AddressValue {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(AddressValueVisitor)
    }
}
