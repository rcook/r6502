use crate::{AddressRange, BusDevice};

pub struct DeviceMapping {
    pub address_range: AddressRange,
    pub device: Box<dyn BusDevice>,
    pub offset: u16,
}
