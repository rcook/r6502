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
        writeln!(f, "  pc: ${pc:04X} ({pc})", pc = self.pc)?;
        writeln!(f, "  s : ${s:02X}  ({s})", s = self.s)?;
        writeln!(f, "  a : ${a:02X}  ({a})", a = self.a)?;
        writeln!(f, "  x : ${x:02X}  ({x})", x = self.x)?;
        writeln!(f, "  y : ${y:02X}  ({y})", y = self.y)?;
        writeln!(f, "        {P_STR}")?;
        writeln!(f, "  p : 0b{p:08b}  (${p:02X}) ({p})", p = self.p.bits())?;

        let mut ram = self.ram.clone();
        ram.sort_by(|a, b| a.address.cmp(&b.address));
        for address_value in &ram {
            writeln!(
                f,
                "    {addr:04X} {value:02X} ({value})",
                addr = address_value.address,
                value = address_value.value
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
