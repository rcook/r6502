use crate::{AddressRange, BusDevice, BusEvent};
use std::sync::mpsc::Sender;

pub(crate) type DeviceFn = Box<dyn FnOnce(Sender<BusEvent>) -> Box<dyn BusDevice>>;

pub(crate) struct DeviceDescription {
    pub(crate) address_range: AddressRange,
    pub(crate) device_fn: DeviceFn,
    pub(crate) offset: u16,
}
