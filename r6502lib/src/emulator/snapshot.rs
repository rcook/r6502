use crate::emulator::util::{make_word, split_word};
use crate::emulator::{Cpu, R6502_DUMP_MAGIC_NUMBERS};
use anyhow::{bail, Result};
use chrono::Utc;
use std::env::current_dir;
use std::fs::{metadata, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

const SNAPSHOT_SIZE: u64 = 0x10000 + 15;

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

    let pc = make_word(bytes[9], bytes[8]);
    let a = bytes[10];
    let x = bytes[11];
    let y = bytes[12];
    let sp = bytes[13];
    let p = bytes[14];

    // etc.
    todo!()
}

pub fn write_snapshot(cpu: &Cpu, path: &Path) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let (hi, lo) = split_word(cpu.reg.pc);
    writer.write_all(&R6502_DUMP_MAGIC_NUMBERS)?;
    writer.write_all(&cpu.bus.machine_tag().unwrap_or([0x00; 4]))?;
    writer.write_all(&[
        lo,
        hi,
        cpu.reg.a,
        cpu.reg.x,
        cpu.reg.y,
        cpu.reg.sp,
        cpu.reg.p.bits(),
    ])?;

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
