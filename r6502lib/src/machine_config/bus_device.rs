use crate::emulator::deserialization::deserialize_word;
use crate::emulator::{
    AddressRange, BusDevice as _BusDevice, BusEvent, DeviceMapping, DummyDevice, Image, Pia, Ram,
    Rom, UiMode,
};
use crate::machine_config::bus_device_type::BusDeviceType;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use std::sync::mpsc::Sender;

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

impl BusDevice {
    pub fn map_device(
        &self,
        ui_mode: UiMode,
        bus_tx: &Sender<BusEvent>,
        images: &[&Image],
    ) -> DeviceMapping {
        let image_slices = images
            .iter()
            .map(|image| image.slice(&self.address_range))
            .collect();
        let device: Box<dyn _BusDevice> = match self.r#type {
            BusDeviceType::Pia => match ui_mode {
                UiMode::Terminal => Box::new(Pia::new(bus_tx.clone())),
                UiMode::Tui => Box::new(DummyDevice),
            },
            BusDeviceType::Ram => Box::new(Ram::new(self.address_range.len(), &image_slices)),
            BusDeviceType::Rom => Box::new(Rom::new(self.address_range.len(), &image_slices)),
        };
        DeviceMapping {
            address_range: self.address_range.clone(),
            device,
            offset: self.offset,
        }
    }
}

fn deserialize_address_range<'de, D>(deserializer: D) -> Result<AddressRange, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(SerdeError::custom)
}
