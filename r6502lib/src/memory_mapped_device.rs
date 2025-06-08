pub trait MemoryMappedDevice: Sync {
    fn start(&self);
    fn load(&self, addr: u16) -> u8;
    fn store(&self, addr: u16, value: u8);
}
