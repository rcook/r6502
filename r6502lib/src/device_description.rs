use crate::{AddressRange, MemoryMappedDevice};

pub(crate) struct DeviceDescription {
    pub(crate) address_range: AddressRange,
    pub(crate) device: Box<dyn FnOnce() -> Box<dyn MemoryMappedDevice>>,
    pub(crate) offset: u16,
}
