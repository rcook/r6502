use crate::ascii::CR;
use crate::emulator::util::make_word;
use crate::emulator::Cpu;
use anyhow::{anyhow, bail, Result};
use log::info;
use path_absolutize::Absolutize;
use std::env::{current_dir, set_current_dir};
use std::fs::{read_dir, File};
use std::io::{BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::Path;

const OSAREG: u16 = 0x00ef; // BASIC zero-page workspace value
const HOOK_OK: u8 = 0;
const HOOK_BRK: u8 = 1;
const CLIV_HOST_HOOK: u8 = 100;
const FILEV_HOST_HOOK: u8 = 101;

pub fn handle_host_hook(cpu: &mut Cpu) -> Result<()> {
    let hook = cpu.bus.load(OSAREG);
    match hook {
        CLIV_HOST_HOOK => handle_cliv(cpu),
        FILEV_HOST_HOOK => handle_filev(cpu),
        _ => bail!("unsupported host hook ${:02X}", cpu.reg.a),
    }
}

fn handle_cliv(cpu: &mut Cpu) -> Result<()> {
    let xy_addr = make_word(cpu.reg.y, cpu.reg.x);
    let command_line = read_cr_terminated_string(cpu, xy_addr)?;

    let (command, arg) = match command_line.split_once(' ') {
        Some((c, a)) => (c.trim(), Some(a.trim())),
        None => (command_line.trim(), None),
    };

    let result = match command {
        "*." | "*CAT" => show_catalogue(arg)?,
        "*DIR" => change_working_dir(arg)?,
        _ => {
            info!("OSCLI command {command_line} not implemented");
            false
        }
    };
    cpu.reg.a = if result { HOOK_OK } else { HOOK_BRK };
    Ok(())
}

fn handle_filev(cpu: &mut Cpu) -> Result<()> {
    let result = match cpu.reg.a {
        0 => save_memory(cpu)?,
        0xff => load_memory(cpu)?,
        _ => {
            info!("OSFILE operation ${a:02X} not implemented", a = cpu.reg.a);
            false
        }
    };
    cpu.reg.a = if result { HOOK_OK } else { HOOK_BRK };
    Ok(())
}

fn show_catalogue(arg: Option<&str>) -> Result<bool> {
    let d = current_dir()?;
    let d = match arg {
        Some(s) => Path::new(s).absolutize_from(&d)?.to_path_buf(),
        None => d,
    };

    let Ok(dir) = read_dir(&d) else {
        return Ok(false);
    };

    println!("Directory: {dir}", dir = d.display());
    for entry in dir {
        let entry = entry?;
        let f = entry.file_name();
        let file_name = f
            .to_str()
            .ok_or_else(|| anyhow!("could not convert file name {f:?}"))?;
        println!("  {file_name}");
    }
    Ok(true)
}

fn change_working_dir(arg: Option<&str>) -> Result<bool> {
    let d = current_dir()?;
    match arg {
        Some(s) => set_current_dir(Path::new(s).absolutize_from(&d)?)?,
        None => {
            println!("{d}", d = d.display());
        }
    }
    Ok(true)
}

fn save_memory(cpu: &mut Cpu) -> Result<bool> {
    let xy_addr = make_word(cpu.reg.y, cpu.reg.x);
    let file_name_addr = read_word(cpu, xy_addr, 0);
    let load_addr = read_dword(cpu, xy_addr, 2);
    let execution_addr = read_dword(cpu, xy_addr, 6);
    let start_addr = read_dword(cpu, xy_addr, 10);
    let end_addr = read_dword(cpu, xy_addr, 14);
    let file_name = read_cr_terminated_string(cpu, file_name_addr)?;

    info!(
        "Saving to {file_name} {load_addr:08X} {execution_addr:08X} {start_addr:08X} {end_addr:08X}",
    );

    let start_addr = u16::try_from(start_addr & 0xffff).unwrap();
    let end_addr = u16::try_from(end_addr & 0xffff).unwrap();

    let d = current_dir()?;
    let p = d.join(&file_name);

    let f = match File::create_new(&p) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                info!("File {path} already exists", path = p.display());
            } else {
                info!(
                    "Error occurred while attempting to save to {path}: {e}",
                    path = p.display()
                );
            }
            return Ok(false);
        }
    };
    let mut writer = BufWriter::new(f);
    for addr in start_addr..end_addr {
        let byte = cpu.bus.load(addr);
        writer.write_all(&[byte])?;
    }

    let length = end_addr - start_addr;

    // TBD: Use my spiffy .inf file writer
    // See https://github.com/rcook/dfstool/blob/main/src/metadata/inf.rs
    let inf_path = d.join(format!("{file_name}.inf"));
    let mut f = match File::create_new(&inf_path) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::AlreadyExists {
                info!("File {path} already exists", path = inf_path.display());
            } else {
                info!(
                    "Error occurred while attempting to save to {path}: {e}",
                    path = inf_path.display()
                );
            }
            return Ok(false);
        }
    };
    writeln!(
        f,
        "\"{file_name}\" {load_addr:08X} {execution_addr:08X} {length:08X} {access:02X}",
        access = 0
    )?;

    info!(
        "Successfully saved to {file_name} {load_addr:08X} {execution_addr:08X} {start_addr:08X} {end_addr:08X}"
    );

    Ok(true)
}

fn load_memory(cpu: &mut Cpu) -> Result<bool> {
    let xy_addr = make_word(cpu.reg.y, cpu.reg.x);
    let file_name_addr = read_word(cpu, xy_addr, 0);
    let load_addr = read_dword(cpu, xy_addr, 2);
    let execution_addr = read_dword(cpu, xy_addr, 6);
    let start_addr = read_dword(cpu, xy_addr, 10);
    let end_addr = read_dword(cpu, xy_addr, 14);
    let file_name = read_cr_terminated_string(cpu, file_name_addr)?;

    info!(
        "Loading {file_name} {load_addr:08X} {execution_addr:08X} {start_addr:08X} {end_addr:08X}",
    );

    let load_addr = u16::try_from(load_addr & 0xffff).unwrap();

    let d = current_dir()?;
    let p = d.join(&file_name);

    let f = match File::open(&p) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                info!("File {path} not found", path = p.display());
            } else {
                info!(
                    "Error occurred while attempting to save to {path}: {e}",
                    path = p.display()
                );
            }
            return Ok(false);
        }
    };
    let mut reader = BufReader::new(f);
    let mut bytes = [0; 1];
    let mut count = 0;
    for addr in load_addr.. {
        match reader.read_exact(&mut bytes) {
            Ok(()) => {}
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    break;
                }
                info!(
                    "Error occurred while attempting to load {path}: {e}",
                    path = p.display()
                );
                return Ok(false);
            }
        }
        cpu.bus.store(addr, bytes[0]);
        count += 1;
    }

    write_dword(cpu, xy_addr, 10, count);

    info!(
        "Successfully loaded {file_name} {load_addr:08X} {execution_addr:08X} {start_addr:08X} {end_addr:08X}",
    );

    Ok(true)
}

fn read_cr_terminated_string(cpu: &Cpu, start_addr: u16) -> Result<String> {
    let mut s = String::with_capacity(200);
    for addr in start_addr.. {
        let byte = cpu.bus.load(addr);
        if byte == CR {
            return Ok(s);
        }
        s.push(byte as char);
    }
    bail!("string at ${:04X} is too long", start_addr)
}

fn read_word(cpu: &mut Cpu, base_addr: u16, offset: u16) -> u16 {
    let lo_addr = base_addr.wrapping_add(offset);
    let hi_addr = lo_addr.wrapping_add(1);
    make_word(cpu.bus.load(hi_addr), cpu.bus.load(lo_addr))
}

fn read_dword(cpu: &mut Cpu, base_addr: u16, offset: u16) -> u32 {
    let b0_addr = base_addr.wrapping_add(offset);
    let b1_addr = b0_addr.wrapping_add(1);
    let b2_addr = b1_addr.wrapping_add(1);
    let b3_addr = b2_addr.wrapping_add(1);
    let b0 = cpu.bus.load(b0_addr) as u32;
    let b1 = cpu.bus.load(b1_addr) as u32;
    let b2 = cpu.bus.load(b2_addr) as u32;
    let b3 = cpu.bus.load(b3_addr) as u32;
    (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
}

fn write_dword(cpu: &mut Cpu, base_addr: u16, offset: u16, value: u32) {
    let b0_addr = base_addr.wrapping_add(offset);
    let b1_addr = b0_addr.wrapping_add(1);
    let b2_addr = b1_addr.wrapping_add(1);
    let b3_addr = b2_addr.wrapping_add(1);
    cpu.bus.store(b0_addr, u8::try_from(value & 0xff).unwrap());
    cpu.bus
        .store(b1_addr, u8::try_from((value >> 8) & 0xff).unwrap());
    cpu.bus
        .store(b2_addr, u8::try_from((value >> 16) & 0xff).unwrap());
    cpu.bus
        .store(b3_addr, u8::try_from((value >> 24) & 0xff).unwrap());
}
