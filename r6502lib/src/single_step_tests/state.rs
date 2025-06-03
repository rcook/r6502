use crate::single_step_tests::AddressValue;
use crate::{P, P_STR};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[derive(Debug, Deserialize)]
pub struct State {
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

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "  pc: ${:04X}", self.pc)?;
        writeln!(f, "  s : ${:02X}", self.s)?;
        writeln!(f, "  a : ${:02X}", self.a)?;
        writeln!(f, "  x : ${:02X}", self.x)?;
        writeln!(f, "  y : ${:02X}", self.y)?;
        writeln!(f, "       {P_STR}")?;
        writeln!(f, "  p : ${:08b}", self.p.bits())?;

        let mut ram = self.ram.clone();
        ram.sort_by(|a, b| a.address.cmp(&b.address));
        for address_value in &ram {
            writeln!(
                f,
                "    {:04X} {:02X}",
                address_value.address, address_value.value
            )?
        }
        Ok(())
    }
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
