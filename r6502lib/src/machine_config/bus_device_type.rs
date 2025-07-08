use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum BusDeviceType {
    #[serde(rename = "pia")]
    Pia,

    #[serde(rename = "ram")]
    Ram,

    #[serde(rename = "rom")]
    Rom,

    #[serde(rename = "via")]
    Via,
}
