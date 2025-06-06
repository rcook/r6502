use crate::util::{make_word, split_word};
use crate::{Image, MEMORY_SIZE};
use anyhow::{bail, Result};
use std::ops::{Index, IndexMut};

pub struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Self([0x00; 0x10000])
    }
}

impl Memory {
    pub fn load(&mut self, image: &Image) -> Result<()> {
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

        self.0[load..limit].copy_from_slice(&image.values);
        Ok(())
    }

    pub fn snapshot(&self, begin: usize, end: usize) -> Vec<u8> {
        self.0[begin..end].to_vec()
    }

    pub fn fetch_word(&self, addr: u16) -> u16 {
        let lo = self[addr];
        let hi = self[addr + 1];
        make_word(hi, lo)
    }

    pub(crate) fn store_word(&mut self, addr: u16, value: u16) {
        let (hi, lo) = split_word(value);
        self[addr] = lo;
        self[addr + 1] = hi;
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
