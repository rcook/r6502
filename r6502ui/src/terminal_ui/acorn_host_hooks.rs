use crate::terminal_ui::RawMode;
use anyhow::{Result, anyhow, bail};
use log::info;
use path_absolutize::Absolutize;
use r6502core::emulator::{BusView, Cpu};
use r6502lib::ascii::CR;
use r6502lib::util::make_word;
use std::env::{current_dir, set_current_dir};
use std::fs::{File, read_dir};
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
        "*SAVE" => save_memory_oscli(cpu, arg)?,
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
        0 => save_memory_osfile(cpu)?,
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

    let mut file_infos = Vec::new();
    for entry in dir {
        let entry = entry?;
        let f = entry.file_name();
        let file_name = f
            .to_str()
            .ok_or_else(|| anyhow!("could not convert file name {f:?}"))?;
        let m = entry.metadata()?;
        file_infos.push((file_name.to_string(), m.len()));
    }
    file_infos.sort();

    let column_width = file_infos.iter().fold(0, |acc, f| acc.max(f.0.len()));

    let raw_mode = RawMode::disable()?;
    println!("Directory: {dir}", dir = d.display());
    for (file_name, len) in file_infos {
        println!("  {file_name:column_width$}  {len:>10}");
    }
    drop(raw_mode);

    Ok(true)
}

fn change_working_dir(arg: Option<&str>) -> Result<bool> {
    let d = current_dir()?;
    if let Some(s) = arg {
        set_current_dir(Path::new(s).absolutize_from(&d)?)?;
    } else {
        let raw_mode = RawMode::disable()?;
        println!("{d}", d = d.display());
        drop(raw_mode);
    }
    Ok(true)
}

fn save_memory_oscli(cpu: &mut Cpu, arg: Option<&str>) -> Result<bool> {
    let Some(arg) = arg else {
        info!("Invalid arguments");
        return Ok(false);
    };

    let tokens = match parse_command_line(arg) {
        Ok(tokens) => tokens,
        Err(e) => {
            info!("Failed to parse arguments {arg}: {e}");
            return Ok(false);
        }
    };

    let mut i = tokens.iter();

    let Some(file_name) = i.next() else {
        info!("Invalid arguments {arg}");
        return Ok(false);
    };

    let Some(start_s) = i.next() else {
        info!("Invalid arguments {arg}");
        return Ok(false);
    };

    let Ok(start) = u32::from_str_radix(start_s, 16) else {
        info!("Invalid start address {start_s}");
        return Ok(false);
    };

    let Some(end_or_len_s) = i.next() else {
        info!("Invalid arguments {arg}");
        return Ok(false);
    };

    let end = if let Some(s) = end_or_len_s.strip_prefix('+') {
        let Ok(len) = u32::from_str_radix(s, 16) else {
            info!("Invalid length {s}");
            return Ok(false);
        };
        start + len
    } else {
        let Ok(end) = u32::from_str_radix(end_or_len_s, 16) else {
            info!("Invalid end address {end_or_len_s}");
            return Ok(false);
        };
        end
    };

    if end <= start {
        info!("Invalid end address or length {end_or_len_s}");
        return Ok(false);
    }

    let exec = match i.next() {
        Some(s) => {
            let Ok(exec) = u32::from_str_radix(s, 16) else {
                info!("Invalid execution address {s}");
                return Ok(false);
            };
            exec
        }
        None => start,
    };

    save_memory(&cpu.bus, file_name, start, exec, start, end)
}

fn save_memory_osfile(cpu: &mut Cpu) -> Result<bool> {
    let xy_addr = make_word(cpu.reg.y, cpu.reg.x);
    let file_name_addr = read_word(cpu, xy_addr, 0);
    let load = read_dword(cpu, xy_addr, 2);
    let exec = read_dword(cpu, xy_addr, 6);
    let start = read_dword(cpu, xy_addr, 10);
    let end = read_dword(cpu, xy_addr, 14);
    let file_name = read_cr_terminated_string(cpu, file_name_addr)?;
    save_memory(&cpu.bus, &file_name, load, exec, start, end)
}

fn save_memory(
    bus: &BusView<'_>,
    file_name: &str,
    load: u32,
    exec: u32,
    start: u32,
    end: u32,
) -> Result<bool> {
    assert!(end > start);
    info!("Saving to {file_name} {load:08X} {exec:08X} {start:08X} {end:08X}",);

    let start = u16::try_from(start & 0xffff).unwrap();
    let end_inclusive = u16::try_from((end - 1) & 0xffff).unwrap();
    let length = usize::from(end_inclusive) - usize::from(start) + 1;

    let d = current_dir()?;
    let p = d.join(file_name);

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
    for addr in start..=end_inclusive {
        let byte = bus.load(addr);
        writer.write_all(&[byte])?;
    }

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
        "\"{file_name}\" {load:08X} {exec:08X} {length:08X} {access:02X}",
        access = 0
    )?;

    info!("Successfully saved to {file_name}");

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
    let b0 = u32::from(cpu.bus.load(b0_addr));
    let b1 = u32::from(cpu.bus.load(b1_addr));
    let b2 = u32::from(cpu.bus.load(b2_addr));
    let b3 = u32::from(cpu.bus.load(b3_addr));
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

#[allow(unused)]
fn parse_command_line(s: &str) -> Result<Vec<String>> {
    let mut i = s.chars().peekable();

    let mut tokens = Vec::new();
    loop {
        while let Some(c) = i.peek() {
            if !c.is_whitespace() {
                break;
            }
            i.next().unwrap();
        }

        match i.next() {
            Some('"') => {
                let mut token = String::new();
                loop {
                    let Some(c) = i.next() else {
                        bail!("syntax error in {s}")
                    };
                    if c == '"' {
                        break;
                    }
                    token.push(c);
                }
                tokens.push(token);
            }
            Some(c) => {
                let mut token = String::new();
                token.push(c);
                while let Some(c) = i.peek() {
                    if c.is_whitespace() {
                        break;
                    }
                    token.push(i.next().unwrap());
                }
                tokens.push(token);
            }
            None => break,
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::terminal_ui::acorn_host_hooks::parse_command_line;
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(&["FILE", "aaa", "bbb", "ccc"], "FILE aaa bbb ccc")]
    #[case(&["FILE NAME", "aaa", "bbb", "ccc"], "\"FILE NAME\" aaa bbb ccc")]
    #[case(&["FILE NAME", "aaa", "bbb", "ccc"], "  \"FILE NAME\"   aaa   bbb   ccc  ")]
    fn parse_command_line_basics(#[case] expected: &[&str], #[case] input: &str) -> Result<()> {
        assert_eq!(expected, parse_command_line(input)?);
        Ok(())
    }
}
