use crate::{BusDevice, ImageSlice};

pub struct Rom<const N: usize> {
    bytes: [u8; N],
}

impl<const N: usize> Rom<N> {
    #[must_use]
    pub fn new(image_slice: &Option<ImageSlice>) -> Self {
        let mut bytes = [0x00; N];
        if let Some(image_slice) = image_slice {
            let load = image_slice.load as usize;
            bytes[load..load + image_slice.bytes.len()].copy_from_slice(image_slice.bytes);
        }
        Self { bytes }
    }
}

impl<const N: usize> BusDevice for Rom<N> {
    fn start(&self) {}

    fn load(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    fn store(&self, _addr: u16, _value: u8) {}

    fn join(&self) {}
}
