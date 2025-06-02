use crate::single_step_tests::AddressValue;
use anyhow::Result;
use serde::de::{Error as SerdeError, SeqAccess, Visitor};
use std::fmt::{Formatter, Result as FmtResult};

pub(crate) struct AddressValueVisitor;

impl<'de> Visitor<'de> for AddressValueVisitor {
    type Value = AddressValue;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("[u16, u8]")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let address = access
            .next_element::<u16>()?
            .ok_or_else(|| SerdeError::custom("Unexpected type"))?;
        let value = access
            .next_element::<u8>()?
            .ok_or_else(|| SerdeError::custom("Unexpected type"))?;
        Ok(AddressValue { address, value })
    }
}
