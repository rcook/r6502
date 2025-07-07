use crate::emulator::r6502_image::R6502ImageType;
use crate::emulator::{Cpu, CpuState, MachineTag, R6502_MAGIC_NUMBER};
use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Snapshot {
    pub machine_tag: MachineTag,
    pub cpu_state: CpuState,
    pub bytes: Vec<u8>,
}

impl Snapshot {
    #[must_use]
    pub fn new(cpu: &Cpu) -> Self {
        Self {
            machine_tag: cpu.bus.machine_tag(),
            cpu_state: CpuState::new(cpu),
            bytes: (0..=0xffff).map(|addr| cpu.bus.load(addr)).collect(),
        }
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&R6502_MAGIC_NUMBER.to_le_bytes())?;
        writer.write_all(&[R6502ImageType::Snapshot as u8])?;
        writer.write_all(&self.machine_tag)?;
        writer.write_all(&self.cpu_state.pc.to_le_bytes())?;
        writer.write_all(&[
            self.cpu_state.a,
            self.cpu_state.x,
            self.cpu_state.y,
            self.cpu_state.sp,
            self.cpu_state.p,
        ])?;
        writer.write_all(&self.cpu_state.total_cycles.to_le_bytes())?;
        writer.write_all(&self.bytes)?;
        Ok(())
    }
}
