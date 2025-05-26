use crate::constants::IRQ_VALUE;
use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{Flag, Memory, Op, State, IRQ, OPS, OSHALT, OSWRCH, STACK_BASE};
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
    // Set up operating system routines
    fn set_brk(state: &mut State, addr: u16) {
        state.store(addr, BRK.opcode); // Software interrupt
        state.store(addr + 1, NOP.opcode); // Padding
        state.store(addr + 2, RTS.opcode); // Return
    }

    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in OPS {
            ops[op.opcode as usize] = Some(op)
        }
        ops
    };

    // Set up interrupt vectors
    state.store_word(IRQ, IRQ_VALUE);

    // Set up operating system handlers
    set_brk(state, OSWRCH);
    set_brk(state, OSHALT);

    // Initialize the state
    state.push_word(OSHALT - 1);

    loop {
        while !state.get_flag(Flag::B) {
            let opcode = state.next();
            match ops[opcode as usize] {
                Some(op) => (op.func)(state),
                None => bail!("Unsupported opcode {opcode:02X}"),
            }
        }

        // Check for expected interrupt request value
        if state.pc != IRQ_VALUE {
            bail!("Unexpected IRQ value {:04X}", state.pc);
        }

        //let p = state.read(STACK_BASE + (state.s + 1) as u16);

        // Address of operating system routine being invoked
        let addr = state.fetch_word(STACK_BASE + (state.s + 2) as u16) - 1;

        match addr {
            OSWRCH => {
                let c = state.a as char;
                state.stdout(c);
            }
            OSHALT => {
                println!("Halted.");
                return Ok(());
            }
            _ => panic!("Break at subroutine {:04X}", addr),
        }

        (RTI.func)(state);
    }
}

pub(crate) fn demo() -> Result<()> {
    let mut state = State::new();
    load(&mut state.memory, Path::new("examples\\Main.bin"), 0x2000)?;
    state.pc = 0x2000u16;
    run(&mut state)?;
    Ok(())
}
