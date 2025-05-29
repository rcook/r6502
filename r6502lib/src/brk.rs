use crate::{Cycles, VmState};

// http://www.6502.org/tutorials/6502opcodes.html#BRK
// http://www.6502.org/users/obelisk/6502/reference.html#BRK
pub(crate) fn brk(_s: &mut VmState) -> Cycles {
    todo!("Need to implement stack pointer etc.")
}

#[cfg(test)]
mod tests {
    use crate::{brk, reg, Memory, VmState};

    #[allow(unused)]
    //#[test]
    fn basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        let cycles = brk(&mut s);
        assert_eq!(7, cycles);
    }
}
