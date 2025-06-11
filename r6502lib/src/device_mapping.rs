use crate::{AddressRange, MemoryMappedDevice};

pub(crate) struct DeviceMapping {
    pub(crate) address_range: AddressRange,
    pub(crate) device: Box<dyn MemoryMappedDevice>,
    pub(crate) offset: u16,
}
