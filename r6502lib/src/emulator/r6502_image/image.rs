use crate::emulator::r6502_image::R6502ImageType;
use crate::emulator::{r6502_image::ImageHeader, Cpu, MachineTag, R6502_MAGIC_NUMBER};
use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, Write};
use std::path::Path;

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
    pub fn new_snapshot(cpu: &Cpu) -> Self {
        let header = ImageHeader::new_snapshot(cpu);
        let bytes = (0..=0xffff).map(|addr| cpu.bus.load(addr)).collect();
        Self { header, bytes }
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
    pub const fn start(&self) -> u16 {
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

    pub fn set_initial_cpu_state(&self, cpu: &mut Cpu) {
        self.header.set_initial_cpu_state(cpu);
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&R6502_MAGIC_NUMBER.to_le_bytes())?;
        writer.write_all(&[R6502ImageType::Snapshot as u8])?;
        self.header.write(&mut writer)?;
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}
