use crate::validation::AddressValueVisitor;
use serde::{Deserialize, Deserializer};
use std::result::Result as StdResult;

#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct AddressValue {
    pub address: u16,
    pub value: u8,
}

impl<'de> Deserialize<'de> for AddressValue {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(AddressValueVisitor)
    }
}
