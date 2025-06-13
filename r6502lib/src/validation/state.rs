use crate::validation::AddressValue;
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct State {
    #[serde(rename = "pc")]
    pub pc: u16,

    #[serde(rename = "s")]
    pub sp: u8,

    #[serde(rename = "a")]
    pub a: u8,

    #[serde(rename = "x")]
    pub x: u8,

    #[serde(rename = "y")]
    pub y: u8,

    #[serde(rename = "p")]
    pub p: u8,

    #[serde(rename = "ram")]
    pub ram: Vec<AddressValue>,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "  pc: ${pc:04X} ({pc})", pc = self.pc)?;
        writeln!(f, "  s : ${s:02X}  ({s})", s = self.sp)?;
        writeln!(f, "  a : ${a:02X}  ({a})", a = self.a)?;
        writeln!(f, "  x : ${x:02X}  ({x})", x = self.x)?;
        writeln!(f, "  y : ${y:02X}  ({y})", y = self.y)?;
        writeln!(
            f,
            "  p : {p} (0b{p_value:08b}) (${p_value:02X}) ({p_value})",
            p = self.p,
            p_value = self.p
        )?;

        let mut ram = self.ram.clone();
        ram.sort_by(|a, b| a.address.cmp(&b.address));
        for address_value in &ram {
            writeln!(
                f,
                "    {addr:04X} {value:02X} ({value})",
                addr = address_value.address,
                value = address_value.value
            )?;
        }
        Ok(())
    }
}

/*
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
*/
