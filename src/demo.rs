use crate::{Memory, OpFn, State, OSHALT};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

fn load(memory: &mut Memory, path: &Path, start: u16) -> Result<()> {
    let len = memory.len();
    let buffer = &mut memory[start as usize..len];
    let mut file = File::open(path)?;
    match file.read_exact(buffer) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
        Err(e) => bail!(e),
    }
    Ok(())
}

fn run(state: &mut State) -> Result<()> {
    use crate::ops::*;

    let mut ops: [Option<OpFn>; 256] = [None; 256];
    ops[0x00] = Some(brk);
    ops[0x20] = Some(jsr);
    ops[0x4c] = Some(jmp_abs);
    ops[0x60] = Some(rts);
    ops[0xa2] = Some(ldx_imm);
    ops[0xbd] = Some(lda_abs_x);
    ops[0xc9] = Some(cmp_imm);
    ops[0xe8] = Some(inx);
    ops[0xf0] = Some(beq);

    // Initialize the state
    state.push_word(OSHALT - 1);

    state.running = true;
    while state.running {
        state.println(&format!("{}", state.dump()));
        let opcode = state.fetch();
        state.println(&format!("opcode {:02X}", opcode));
        match ops[opcode as usize] {
            Some(op_fn) => op_fn(state),
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
