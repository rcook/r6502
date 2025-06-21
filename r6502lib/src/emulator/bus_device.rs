pub trait BusDevice {
    fn start(&self) {}

    #[must_use]
    fn stop(&self) -> bool {
        true
    }

    fn load(&self, addr: u16) -> u8;

    fn store(&self, addr: u16, value: u8);
}
