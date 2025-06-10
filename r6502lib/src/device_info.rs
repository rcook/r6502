use crate::MemoryMappedDevice;

pub(crate) struct DeviceInfo {
    pub(crate) start: u16,
    pub(crate) end: u16,
    pub(crate) device: Box<dyn MemoryMappedDevice>,
    pub(crate) offset: u16,
}
