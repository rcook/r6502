use crate::{Flag, Memory, OpFn, State, OSHALT, OSWRCH};
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
    /* 0x00 */
    fn brk(state: &mut State) {
        let pc = state.pc - 1;
        match pc {
            OSWRCH => {
                let c = state.a as char;
                state.stdout(c);
                rts(state);
            }
            OSHALT => {
                state.running = false;
            }
            _ => panic!("Break at {:04X}", pc),
        }
    }

    /* 0x20 */
    fn jsr(state: &mut State) {
        let addr = state.fetch_word();
        state.push_word(state.pc - 1);
        state.pc = addr;
    }

    /* 0x4c */
    fn jmp_abs(state: &mut State) {
        state.pc = state.fetch_word();
    }

    /* 0x60 */
    fn rts(state: &mut State) {
        state.pc = state.pull_word();
        state.pc += 1;
    }

    /* 0xa2 */
    fn ldx_imm(state: &mut State) {
        let value = state.fetch();
        state.x = value;
    }

    /* 0xbd */
    fn lda_abs_x(state: &mut State) {
        let base_addr = state.fetch_word();
        let addr = base_addr + state.x as u16;
        let value = state.memory[addr as usize];
        state.a = value;
    }

    /* 0xc9 */
    fn cmp_imm(state: &mut State) {
        let value = state.fetch();
        let result = state.a as i32 - value as i32;
        state.set_flag(Flag::N, state.a >= 0x80u8);
        state.set_flag(Flag::Z, result == 0);
        state.set_flag(Flag::CARRY, result >= 0);
    }

    /* 0xe8 */
    fn inx(state: &mut State) {
        state.x += 1;
    }

    /* 0xf0 */
    fn beq(state: &mut State) {
        let value = state.fetch();
        if state.get_flag(Flag::Z) {
            match state.pc.checked_add(value as u16) {
                Some(result) => state.pc = result,
                None => todo!(),
            }
        }
    }

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
