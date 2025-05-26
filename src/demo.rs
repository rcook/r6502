use crate::{Memory, Op, State, OPS, OSHALT};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

fn load(memory: &mut Memory, path: &Path, addr: u16) -> Result<()> {
    let len = memory.len();
    let buffer = &mut memory[addr as usize..len];
    let mut file = File::open(path)?;
    match file.read_exact(buffer) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
        Err(e) => bail!(e),
    }
    Ok(())
}

fn run(state: &mut State) -> Result<()> {
    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in OPS {
            ops[op.opcode as usize] = Some(op)
        }
        ops
    };

    // Initialize the state
    state.push_word(OSHALT - 1);

    state.running = true;
    while state.running {
        state.println(&format!("{}", state.dump()));
        let opcode = state.fetch();
        state.println(&format!("opcode {:02X}", opcode));
        match ops[opcode as usize] {
            Some(op) => (op.func)(state),
            None => todo!("opcode {opcode:02X} not implemented"),
        }
    }
    Ok(())
}

pub(crate) fn demo() -> Result<()> {
    let mut state = State::new();
    load(&mut state.memory, Path::new("examples\\Main.bin"), 0x2000)?;
    state.pc = 0x2000u16;
    run(&mut state)?;
    Ok(())
}
