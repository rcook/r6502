use anyhow::{bail, Result};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;

const N_MASK: u8 = 0b1000_0000u8;
const Z_MASK: u8 = 0b0000_0010u8;
const CARRY_MASK: u8 = 0b0000_0001u8;
const OSWRCH: u16 = 0xffeeu16;
const OSHALT: u16 = 0xfffeu16;

type Memory = [u8; 0x10000];
type OpFn = fn(&mut State) -> ();

struct State {
    p: u8,
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    memory: Memory,
    running: bool,
}

impl State {
    fn new() -> Self {
        Self {
            pc: 0x0000u16,
            p: 0x00u8,
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            s: 0xffu8,
            memory: [0x00u8; 0x10000],
            running: false,
        }
    }

    fn dump(&self) -> String {
        format!(
            "pc={:04X} NV1BDIZC={:08b} a={:02X} x={:02X} y={:02X} s={:02X}",
            self.pc, self.p, self.a, self.x, self.y, self.s,
        )
    }

    fn println(&self, _s: &str) {
        //println!("{s}");
    }

    fn stdout(&self, c: char) {
        print!("{c}")
    }
}

macro_rules! fetch {
    ($state: expr) => {{
        let value = $state.memory[$state.pc as usize];
        $state.println(&format!("FETCH {:04X} -> {:02X}", $state.pc, value));
        $state.pc += 1;
        value
    }};
}

macro_rules! get_p {
    ($state: expr, $mask: expr) => {
        ($state.p & $mask) != 0x00u8
    };
}

macro_rules! set_p {
    ($state: expr, $mask: expr, $value: expr) => {
        if $value {
            $state.p |= $mask
        } else {
            $state.p &= !$mask
        }
    };
}

macro_rules! z {
    ($state: expr) => {
        get_p!($state, Z_MASK)
    };
}

macro_rules! set_n {
    ($state: expr, $value: expr) => {
        set_p!($state, N_MASK, $value);
    };
}

macro_rules! set_z {
    ($state: expr, $value: expr) => {
        set_p!($state, Z_MASK, $value);
    };
}

macro_rules! set_carry {
    ($state: expr, $value: expr) => {
        set_p!($state, CARRY_MASK, $value);
    };
}

macro_rules! push {
    ($state: expr, $value: expr) => {{
        let addr = 0x0100u16 + $state.s as u16;
        $state.memory[addr as usize] = $value;
        $state.println(&format!("push {:04X} <- {:02X}", addr, $value));
        $state.s -= 1;
    }};
}

macro_rules! push_word {
    ($state: expr, $value: expr) => {
        push!($state, ($value >> 8) as u8);
        push!($state, $value as u8);
    };
}

macro_rules! push_ret_addr {
    ($state: expr, $value: expr) => {
        push_word!($state, $value - 1)
    };
}

macro_rules! pull {
    ($state: expr) => {{
        $state.s += 1;
        let addr = 0x0100u16 + $state.s as u16;
        let value = $state.memory[addr as usize];
        $state.println(&format!("pull {:04X} -> {:02X}", addr, value));
        value
    }};
}

macro_rules! pull_word {
    ($state: expr) => {{
        let lo = pull!($state);
        let hi = pull!($state);
        ((hi as u16) << 8) + lo as u16
    }};
}

macro_rules! pull_ret_addr {
    ($state: expr) => {
        pull_word!($state) + 1
    };
}

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
        let lo = fetch!(state);
        let hi = fetch!(state);
        let addr = ((hi as u16) << 8) + lo as u16;
        push_ret_addr!(state, state.pc);
        state.pc = addr;
    }

    /* 0x4c */
    fn jmp_abs(state: &mut State) {
        let lo = fetch!(state);
        let hi = fetch!(state);
        state.pc = ((hi as u16) << 8) + lo as u16;
    }

    /* 0x60 */
    fn rts(state: &mut State) {
        state.pc = pull_ret_addr!(state);
    }

    /* 0xa2 */
    fn ldx_imm(state: &mut State) {
        let value = fetch!(state);
        state.x = value;
    }

    /* 0xbd */
    fn lda_abs_x(state: &mut State) {
        let lo = fetch!(state);
        let hi = fetch!(state);
        let base_addr = ((hi as u16) << 8) + lo as u16;
        let addr = base_addr + state.x as u16;
        let value = state.memory[addr as usize];
        state.a = value;
    }

    /* 0xc9 */
    fn cmp_imm(state: &mut State) {
        let value = fetch!(state);
        let result = state.a as i32 - value as i32;
        set_n!(state, state.a >= 0x80u8);
        set_z!(state, result == 0);
        set_carry!(state, result >= 0);
    }

    /* 0xe8 */
    fn inx(state: &mut State) {
        state.x += 1;
    }

    /* 0xf0 */
    fn beq(state: &mut State) {
        let value = fetch!(state);
        if z!(state) {
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
    push_word!(state, OSHALT - 1);

    state.running = true;
    while state.running {
        state.println(&format!("{}", state.dump()));
        let opcode = fetch!(state);
        state.println(&format!("opcode {:02X}", opcode));
        match ops[opcode as usize] {
            Some(op_fn) => op_fn(state),
            None => todo!("opcode {opcode:02X} not implemented"),
        }
    }
    Ok(())
}

fn demo() -> Result<()> {
    let mut state = State::new();
    load(&mut state.memory, Path::new("examples\\Main.bin"), 0x2000)?;
    state.pc = 0x2000u16;
    run(&mut state)?;
    Ok(())
}

fn main() -> Result<()> {
    demo()
}
