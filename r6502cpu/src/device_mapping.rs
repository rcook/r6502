use crate::BusDevice;
use r6502core::AddressRange;

pub struct DeviceMapping {
    pub address_range: AddressRange,
    pub device: Box<dyn BusDevice>,
    pub offset: u16,
}
