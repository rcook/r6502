use crate::{AddressRange, BusDevice};

pub(crate) type DeviceFn = Box<dyn FnOnce() -> Box<dyn BusDevice>>;

pub(crate) struct DeviceDescription {
    pub(crate) address_range: AddressRange,
    pub(crate) device_fn: DeviceFn,
    pub(crate) offset: u16,
}
