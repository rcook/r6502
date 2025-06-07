use crate::util::make_word;
use crate::{p_set, VmState, IRQ, P};

// http://www.6502.org/tutorials/6502opcodes.html#BRK
// http://www.6502.org/users/obelisk/6502/reference.html#BRK
// http://forum.6502.org/viewtopic.php?t=6099
// https://www.pagetable.com/c64ref/6502/?tab=2#BRK
// https://www.nesdev.org/wiki/Visual6502wiki/6502_BRK_and_B_bit
// https://retrocomputing.stackexchange.com/questions/12291/what-are-uses-of-the-byte-after-brk-instruction-on-6502
// https://retrocomputing.stackexchange.com/questions/29923/why-does-the-brk-instruction-set-the-b-flag
// https://forums.nesdev.org/viewtopic.php?p=64224#p64224
// https://www.pagetable.com/?p=410
pub(crate) fn brk(s: &mut VmState) {
    s.push_word(s.reg.pc + 1);
    s.push((s.reg.p | P::B).bits());
    s.reg.pc = make_word(s.memory.load(IRQ.wrapping_add(1)), s.memory.load(IRQ));
    p_set!(s.reg, I, true);
}

// http://www.6502.org/tutorials/6502opcodes.html#NOP
// http://www.6502.org/users/obelisk/6502/reference.html#NOP
pub(crate) fn nop(_s: &mut VmState) {}

#[cfg(test)]
mod tests {
    use crate::ops::brk;
    use crate::util::split_word;
    use crate::{Memory, Reg, VmState, _p, IRQ, STACK_BASE};
    use anyhow::Result;
    use rstest::rstest;

    #[rstest]
    #[case(0x1234, 0xfc, 0b10101110, 0x0100, 0xff, 0b10101010, 0x1234)]
    // cargo run -p r6502validation -- run-json '{ "name": "00 3f f7", "initial": { "pc": 35714, "s": 81, "a": 203, "x": 117, "y": 162, "p": 106, "ram": [ [35714, 0], [35715, 63], [35716, 247], [65534, 212], [65535, 37], [9684, 237]]}, "final": { "pc": 9684, "s": 78, "a": 203, "x": 117, "y": 162, "p": 110, "ram": [ [335, 122], [336, 132], [337, 139], [9684, 237], [35714, 0], [35715, 63], [35716, 247], [65534, 212], [65535, 37]]}, "cycles": [ [35714, 0, "read"], [35715, 63, "read"], [337, 139, "write"], [336, 132, "write"], [335, 122, "write"], [65534, 212, "read"], [65535, 37, "read"]] }'
    #[case(0x25d4, 78, 110, 0x8b82, 81, 106, 0x25d4)]
    fn brk_basics(
        #[case] expected_pc: u16,
        #[case] expected_s: u8,
        #[case] expected_p: u8,
        #[case] pc: u16,
        #[case] sp: u8,
        #[case] p: u8,
        #[case] irq_addr: u16,
    ) -> Result<()> {
        let memory = Memory::default();
        let mut s = VmState::new(Reg::default(), memory.view());
        s.reg.p = _p!(p);
        s.reg.pc = pc + 1;
        s.reg.sp = sp;
        let (hi, lo) = split_word(irq_addr);
        s.memory.store(IRQ, lo);
        s.memory.store(IRQ.wrapping_add(1), hi);
        brk(&mut s);
        assert_eq!(_p!(expected_p), s.reg.p);
        assert_eq!(expected_pc, s.reg.pc);
        assert_eq!(expected_s, s.reg.sp);
        assert_eq!(
            p | 0b00010000,
            s.memory
                .load(STACK_BASE.wrapping_add(expected_s as u16).wrapping_add(1))
        ); // P with B flag set
        Ok(())
    }
}
