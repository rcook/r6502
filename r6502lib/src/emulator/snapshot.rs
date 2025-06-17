use crate::emulator::{Cpu, TotalCycles, R6502_DUMP_MAGIC_NUMBERS};
use anyhow::{bail, Result};
use chrono::Utc;
use std::env::current_dir;
use std::fs::{metadata, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

const SNAPSHOT_SIZE: u64 = 0x10000 + 23;

#[allow(clippy::many_single_char_names)]
#[allow(unused)]
pub fn read_snapshot(path: &Path) -> Result<()> {
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

    let machine_tag = &bytes[4..8];
    let pc = u16::from_le_bytes(<[u8; 2]>::try_from(&bytes[8..10]).unwrap());
    let a = bytes[10];
    let x = bytes[11];
    let y = bytes[12];
    let sp = bytes[13];
    let p = bytes[14];
    let total_cycles = TotalCycles::from_le_bytes(<[u8; 8]>::try_from(&bytes[15..23]).unwrap());

    // etc.
    todo!()
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
