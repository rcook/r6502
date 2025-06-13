pub trait BusDevice {
    fn start(&self) {}
    fn stop(&self) {}
    fn load(&self, addr: u16) -> u8;
    fn store(&self, addr: u16, value: u8);
}
