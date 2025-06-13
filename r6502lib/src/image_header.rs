use crate::{ImageFormat, MachineTag};

pub(crate) struct ImageHeader {
    pub(crate) format: ImageFormat,
    pub(crate) tag: Option<MachineTag>,
    pub(crate) load: u16,
    pub(crate) start: u16,
    pub(crate) sp: u8,
}
