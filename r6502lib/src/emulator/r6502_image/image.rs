use crate::emulator::r6502_image::ImageHeader;
use crate::emulator::{Cpu, CpuState, MachineTag};
use anyhow::Result;
use std::io::{Read, Seek};

pub struct Image {
    header: ImageHeader,
    bytes: Vec<u8>,
}

impl Image {
    pub fn try_from_reader<R: Read + Seek>(reader: &mut R) -> Result<Option<Self>> {
        let Some(header) = ImageHeader::try_from_reader(reader)? else {
            return Ok(None);
        };
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(Some(Self { header, bytes }))
    }

    #[must_use]
    pub const fn machine_tag(&self) -> MachineTag {
        self.header.machine_tag()
    }

    #[must_use]
    pub const fn load(&self) -> Option<u16> {
        self.header.load()
    }

    #[must_use]
    pub const fn start(&self) -> Option<u16> {
        self.header.start()
    }

    #[must_use]
    pub const fn sp(&self) -> Option<u8> {
        self.header.sp()
    }

    #[must_use]
    pub const fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }

    #[must_use]
    pub fn get_initial_cpu_state(&self, cpu: &Cpu) -> CpuState {
        self.header.get_initial_cpu_state(cpu)
    }
}
