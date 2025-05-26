use crate::{Flag, State, OSHALT, OSWRCH};

/* 0x00 */
pub(crate) fn brk(state: &mut State) {
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
pub(crate) fn jsr(state: &mut State) {
    let addr = state.fetch_word();
    state.push_word(state.pc - 1);
    state.pc = addr;
}

/* 0x4c */
pub(crate) fn jmp_abs(state: &mut State) {
    state.pc = state.fetch_word();
}

/* 0x60 */
pub(crate) fn rts(state: &mut State) {
    state.pc = state.pull_word();
    state.pc += 1;
}

/* 0xa2 */
pub(crate) fn ldx_imm(state: &mut State) {
    let value = state.fetch();
    state.x = value;
}

/* 0xbd */
pub(crate) fn lda_abs_x(state: &mut State) {
    let base_addr = state.fetch_word();
    let addr = base_addr + state.x as u16;
    let value = state.memory[addr as usize];
    state.a = value;
}

/* 0xc9 */
pub(crate) fn cmp_imm(state: &mut State) {
    let value = state.fetch();
    let result = state.a as i32 - value as i32;
    state.set_flag(Flag::N, state.a >= 0x80u8);
    state.set_flag(Flag::Z, result == 0);
    state.set_flag(Flag::CARRY, result >= 0);
}

/* 0xe8 */
pub(crate) fn inx(state: &mut State) {
    state.x += 1;
}

/* 0xf0 */
pub(crate) fn beq(state: &mut State) {
    let value = state.fetch();
    if state.get_flag(Flag::Z) {
        match state.pc.checked_add(value as u16) {
            Some(result) => state.pc = result,
            None => todo!(),
        }
    }
}
