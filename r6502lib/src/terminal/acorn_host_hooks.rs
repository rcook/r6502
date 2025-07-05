use crate::ascii::CR;
use crate::emulator::util::make_word;
use crate::emulator::Cpu;
use anyhow::{bail, Result};

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
    todo!("command line: [{s}]")
}

fn handle_filev(_cpu: &mut Cpu) -> Result<()> {
    todo!()
}
