use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub enum OutputDeviceType {
    #[default]
    #[serde(rename = "simple")]
    Simple,

    #[serde(rename = "vdu-driver")]
    VduDriver,
}
