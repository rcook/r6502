use crate::{AddressRange, BusDevice};

pub(crate) struct DeviceDescription {
    pub(crate) address_range: AddressRange,
    pub(crate) device: Box<dyn FnOnce() -> Box<dyn BusDevice>>,
    pub(crate) offset: u16,
}
