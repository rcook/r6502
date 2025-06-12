use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum BusDeviceType {
    #[serde(rename = "pia")]
    Pia,

    #[serde(rename = "ram")]
    Ram,

    #[serde(rename = "rom")]
    Rom,
}
