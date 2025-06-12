use crate::{BusDevice, ImageSlice};
use std::sync::atomic::{AtomicU8, Ordering};

pub struct Ram<const N: usize> {
    bytes: [AtomicU8; N],
}

impl<const N: usize> Ram<N> {
    #[must_use]
    pub fn new(image_slice: &Option<ImageSlice>) -> Self {
        let bytes = [const { AtomicU8::new(0x00) }; N];
        if let Some(image_slice) = image_slice {
            let load = image_slice.load as usize;
            for (i, value) in image_slice.bytes.iter().enumerate() {
                bytes[load + i].store(*value, Ordering::SeqCst);
            }
        }
        Self { bytes }
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
