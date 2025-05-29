use crate::{set, Cycles, VmState, IRQ};

// http://www.6502.org/tutorials/6502opcodes.html#BRK
// http://www.6502.org/users/obelisk/6502/reference.html#BRK
pub(crate) fn brk(s: &mut VmState) -> Cycles {
    s.push_word(s.reg.pc);
    s.push(s.reg.p.bits());
    s.reg.pc = s.memory.fetch_word(IRQ);
    set!(s.reg, B, true);
    7
}

#[cfg(test)]
mod tests {
    use crate::{brk, get, reg, Memory, VmState, IRQ};

    #[test]
    fn basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        s.memory.store_word(IRQ, 0x1234);
        assert!(!get!(s.reg, B));
        assert_eq!(0x0000, s.reg.pc);
        assert_eq!(0xff, s.reg.s);
        let cycles = brk(&mut s);
        assert_eq!(7, cycles);
        assert!(get!(s.reg, B));
        assert_eq!(0x1234, s.reg.pc);
        assert_eq!(0xfc, s.reg.s);
    }
}
