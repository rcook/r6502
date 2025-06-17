use crate::emulator::r6502_image::ImageHeader;
use crate::emulator::{Cpu, MachineTag, TotalCycles, MEMORY_SIZE, R6502_DUMP_MAGIC_NUMBERS};
use anyhow::{bail, Result};
use chrono::Utc;
use std::env::current_dir;
use std::fs::{metadata, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

const SNAPSHOT_SIZE: u64 = MEMORY_SIZE as u64 + 23;

pub struct Snapshot {
    pub header: ImageHeader,
    pub bytes: [u8; MEMORY_SIZE],
}

impl Snapshot {
    #[allow(unused)]
    #[allow(clippy::many_single_char_names)]
    pub fn read(path: &Path) -> Result<Self> {
        let m = metadata(path)?;
        if m.len() != SNAPSHOT_SIZE {
            bail!("Snapshot {path} is invalid", path = path.display())
        }

        let mut file = File::open(path)?;
        let mut bytes = Vec::with_capacity(m.len() as usize);
        file.read_to_end(&mut bytes)?;

        if bytes[0..4] != R6502_DUMP_MAGIC_NUMBERS {
            bail!("Snapshot {path} is invalid", path = path.display())
        }

        let machine_tag = <MachineTag>::try_from(&bytes[4..8]).unwrap();
        let pc = u16::from_le_bytes(<[u8; 2]>::try_from(&bytes[8..10]).unwrap());
        let a = bytes[10];
        let x = bytes[11];
        let y = bytes[12];
        let sp = bytes[13];
        let p = bytes[14];
        let total_cycles = TotalCycles::from_le_bytes(<[u8; 8]>::try_from(&bytes[15..23]).unwrap());
        let bytes = <[u8; MEMORY_SIZE]>::try_from(&bytes[23..]).unwrap();
        todo!();
        /*
        Ok(Self {
            header: R6502SnapshotHeader {
                machine_tag,
                pc,
                a,
                x,
                y,
                sp,
                p,
                total_cycles,
            },
            bytes,
        })
        */
    }
}

pub fn write_snapshot(cpu: &Cpu, path: &Path) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&R6502_DUMP_MAGIC_NUMBERS)?;
    writer.write_all(&cpu.bus.machine_tag().unwrap_or([0x00; 4]))?;
    writer.write_all(&cpu.reg.pc.to_le_bytes())?;
    writer.write_all(&[
        cpu.reg.a,
        cpu.reg.x,
        cpu.reg.y,
        cpu.reg.sp,
        cpu.reg.p.bits(),
    ])?;
    writer.write_all(&cpu.total_cycles.to_le_bytes())?;

    // There must be a more efficient way of doing this...
    for addr in 0..=0xffff {
        writer.write_all(&[cpu.bus.load(addr)])?;
    }

    drop(writer);

    let m = metadata(path)?;
    if m.len() != SNAPSHOT_SIZE {
        bail!("Snapshot {path} is invalid", path = path.display())
    }

    Ok(())
}

pub fn write_snapshot_with_unique_name(cpu: &Cpu) -> Result<()> {
    let now = Utc::now();
    let file_name = format!(
        "r6502-snapshot-{timestamp}.bin",
        timestamp = now.format("%Y%m%d%H%M%S")
    );

    let path = current_dir()?.join(file_name);
    write_snapshot(cpu, &path)?;
    Ok(())
}
