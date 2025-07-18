use crate::Cycle;
use serde::de::{Error as SerdeError, SeqAccess, Visitor};
use std::fmt::{Formatter, Result as FmtResult};
use std::result::Result as StdResult;

pub struct CycleVisitor;

impl<'de> Visitor<'de> for CycleVisitor {
    type Value = Cycle;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("[u16, u8, String]")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> StdResult<Self::Value, A::Error> {
        let address = access
            .next_element::<u16>()?
            .ok_or_else(|| SerdeError::custom("Unexpected type"))?;
        let value = access
            .next_element::<u8>()?
            .ok_or_else(|| SerdeError::custom("Unexpected type"))?;
        let operation = access
            .next_element::<String>()?
            .ok_or_else(|| SerdeError::custom("Unexpected type"))?;
        Ok(Cycle {
            address,
            value,
            operation,
        })
    }
}
