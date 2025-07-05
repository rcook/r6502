use crate::ascii::CR;
use crate::emulator::util::make_word;
use crate::emulator::Cpu;
use anyhow::{anyhow, bail, Result};
use std::env::current_dir;
use std::fs::read_dir;

const HOOK_OK: u8 = 0;
const HOOK_BRK: u8 = 1;
const CLIV_HOST_HOOK: u8 = 100;
const FILEV_HOST_HOOK: u8 = 101;

pub fn handle_host_hook(cpu: &mut Cpu) -> Result<()> {
    match cpu.reg.a {
        CLIV_HOST_HOOK => handle_cliv(cpu),
        FILEV_HOST_HOOK => handle_filev(cpu),
        _ => bail!("unsupported host hook ${:02X}", cpu.reg.a),
    }
}

fn handle_cliv(cpu: &mut Cpu) -> Result<()> {
    let command_line_addr = make_word(cpu.reg.y, cpu.reg.x);
    let mut s = String::with_capacity(200);
    for addr in command_line_addr.. {
        let byte = cpu.bus.load(addr);
        if byte == CR {
            break;
        }
        s.push(byte as char);
    }

    match s.as_str() {
        "*." | "*CAT" => {
            show_catalogue()?;
            cpu.reg.a = HOOK_OK;
        }
        _ => cpu.reg.a = HOOK_BRK, // "Bad command"
    }
    Ok(())
}

fn handle_filev(_cpu: &mut Cpu) -> Result<()> {
    todo!()
}

fn show_catalogue() -> Result<()> {
    let d = current_dir()?;
    println!("Directory: {dir}", dir = d.display());
    for entry in read_dir(d)? {
        let entry = entry?;
        let f = entry.file_name();
        let file_name = f
            .to_str()
            .ok_or_else(|| anyhow!("could not convert file name {f:?}"))?;
        println!("  {f}", f = file_name);
    }
    Ok(())
}
