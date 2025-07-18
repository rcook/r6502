use crate::{BusDeviceType, deserialize_address_range, deserialize_word};
use r6502core::AddressRange;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BusDevice {
    #[serde(rename = "type")]
    pub r#type: BusDeviceType,

    #[serde(
        rename = "addressRange",
        deserialize_with = "deserialize_address_range"
    )]
    pub address_range: AddressRange,

    #[serde(rename = "offset", deserialize_with = "deserialize_word")]
    pub offset: u16,
}
