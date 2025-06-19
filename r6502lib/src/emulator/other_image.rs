use crate::emulator::{Cpu, OtherImageHeader};
use anyhow::Result;
use std::io::{Read, Seek};

pub struct OtherImage {
    header: OtherImageHeader,
    bytes: Vec<u8>,
}

impl OtherImage {
    pub fn from_reader<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let header = OtherImageHeader::from_reader(reader)?;
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(Self { header, bytes })
    }

    #[must_use]
    pub const fn new_sim6502(load: u16, start: u16, sp: u8, bytes: Vec<u8>) -> Self {
        Self {
            header: OtherImageHeader::Sim6502 { load, start, sp },
            bytes,
        }
    }

    #[must_use]
    pub const fn new_listing(load: u16, start: u16, bytes: Vec<u8>) -> Self {
        Self {
            header: OtherImageHeader::Listing { load, start },
            bytes,
        }
    }

    #[must_use]
    pub const fn new_raw(bytes: Vec<u8>) -> Self {
        Self {
            header: OtherImageHeader::Raw,
            bytes,
        }
    }

    #[must_use]
    pub const fn load(&self) -> Option<u16> {
        match self.header {
            OtherImageHeader::Sim6502 { load, .. } | OtherImageHeader::Listing { load, .. } => {
                Some(load)
            }
            OtherImageHeader::Raw => None,
        }
    }

    #[must_use]
    pub const fn start(&self) -> Option<u16> {
        match self.header {
            OtherImageHeader::Sim6502 { start, .. }
            | OtherImageHeader::Listing { load: _, start } => Some(start),
            OtherImageHeader::Raw => None,
        }
    }

    #[must_use]
    pub const fn sp(&self) -> Option<u8> {
        match self.header {
            OtherImageHeader::Sim6502 { sp, .. } => Some(sp),
            OtherImageHeader::Listing { .. } | OtherImageHeader::Raw => None,
        }
    }

    #[must_use]
    pub const fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub const fn set_initial_cpu_state(&self, cpu: &mut Cpu) {
        self.header.set_initial_cpu_state(cpu);
    }
}
