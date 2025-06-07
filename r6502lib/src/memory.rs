use crate::constants::NMI;
use crate::util::make_word;
use crate::{Image, MemoryView, IRQ, MEMORY_SIZE, RESET};
use anyhow::{bail, Result};
use std::sync::atomic::{AtomicU8, Ordering};

pub struct Memory([AtomicU8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self([const { AtomicU8::new(0x00) }; MEMORY_SIZE])
    }

    // Don't call this when more than one thread is concurrently accessing memory
    pub fn load_nmi_unsafe(&self) -> u16 {
        let lo = self.load(NMI);
        let hi = self.load(NMI + 1);
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    pub fn load_reset_unsafe(&self) -> u16 {
        let lo = self.load(RESET);
        let hi = self.load(RESET + 1);
        make_word(hi, lo)
    }

    // Don't call this when more than one thread is concurrently accessing memory
    pub fn load_irq_unsafe(&self) -> u16 {
        let lo = self.load(IRQ);
        let hi = self.load(IRQ + 1);
        make_word(hi, lo)
    }

    pub fn store_image(&self, image: &Image) -> Result<()> {
        let load = image.load as usize;
        if load > self.0.len() {
            bail!("Load address ${load:04X} out of range");
        }

        let limit = load + image.values.len();
        if limit > self.0.len() {
            bail!(
                "Image size ${size:04X} starting at load address ${load:04X} is too big",
                size = image.values.len()
            );
        }

        // Cannot do this unfortunately!
        // self.0[load..limit].copy_from_slice(&image.values);
        for (i, value) in image.values.iter().enumerate() {
            self.0[i + load].store(*value, Ordering::Relaxed)
        }

        Ok(())
    }

    pub fn snapshot(&self, begin: usize, end: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(end - begin);
        for value in self.0[begin..end].iter() {
            result.push(value.load(Ordering::Relaxed));
        }
        result
    }

    pub fn view(&self) -> MemoryView {
        MemoryView::new(self)
    }

    #[cfg(not(feature = "apple1"))]
    pub fn load(&self, addr: u16) -> u8 {
        self.0[addr as usize].load(Ordering::Relaxed)
    }

    #[cfg(feature = "apple1")]
    pub fn load(&self, addr: u16) -> u8 {
        // Ugly hack!
        if addr == 0xd010 {
            let temp = self.0[addr as usize].swap(0x00, Ordering::Relaxed);
            self.0[0xd011].store(0x00, Ordering::Relaxed);
            temp
        } else {
            self.0[addr as usize].load(Ordering::Relaxed)
        }
    }

    pub fn store(&self, addr: u16, value: u8) {
        self.0[addr as usize].store(value, Ordering::Relaxed)
    }
}
