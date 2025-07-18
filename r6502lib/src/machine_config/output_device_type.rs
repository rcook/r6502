use crate::emulator::OutputDevice;
use crate::terminal_ui::{SimpleOutput, VduDriver};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub enum OutputDeviceType {
    #[default]
    #[serde(rename = "simple")]
    Simple,

    #[serde(rename = "vdu-driver")]
    VduDriver,
}

impl OutputDeviceType {
    pub fn create_output_device(&self) -> Box<dyn OutputDevice> {
        match self {
            Self::Simple => Box::new(SimpleOutput),
            Self::VduDriver => Box::new(VduDriver::default()),
        }
    }
}
