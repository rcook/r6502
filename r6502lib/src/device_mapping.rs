use crate::{AddressRange, BusDevice};

pub(crate) struct DeviceMapping {
    pub(crate) address_range: AddressRange,
    pub(crate) device: Box<dyn BusDevice>,
    pub(crate) offset: u16,
}
