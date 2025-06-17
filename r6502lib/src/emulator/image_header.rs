use crate::emulator::{ImageFormat, MachineTag};

pub struct ImageHeader {
    pub format: ImageFormat,
    pub machine_tag: Option<MachineTag>,
    pub load: u16,
    pub start: u16,
    pub sp: u8,
}
