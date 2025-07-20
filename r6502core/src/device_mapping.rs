use crate::BusDevice;
use r6502lib::AddressRange;

pub struct DeviceMapping {
    pub address_range: AddressRange,
    pub device: Box<dyn BusDevice>,
    pub offset: u16,
}
