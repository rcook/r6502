use crate::BusDevice;
use std::sync::atomic::{AtomicU8, Ordering};

pub struct Ram<const N: usize> {
    bytes: [AtomicU8; N],
}

impl<const N: usize> Default for Ram<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Ram<N> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bytes: [const { AtomicU8::new(0x00) }; N],
        }
    }
}

impl<const N: usize> BusDevice for Ram<N> {
    fn start(&self) {}

    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize].load(Ordering::SeqCst)
    }

    fn store(&self, addr: u16, value: u8) {
        self.bytes[addr as usize].store(value, Ordering::SeqCst);
    }

    fn join(&self) {}
}
