use crate::{p_set, VmState, IRQ};

// http://www.6502.org/tutorials/6502opcodes.html#BRK
// http://www.6502.org/users/obelisk/6502/reference.html#BRK
// http://forum.6502.org/viewtopic.php?t=6099
// https://www.pagetable.com/c64ref/6502/?tab=2#BRK
// https://www.nesdev.org/wiki/Visual6502wiki/6502_BRK_and_B_bit
// https://retrocomputing.stackexchange.com/questions/12291/what-are-uses-of-the-byte-after-brk-instruction-on-6502
// https://retrocomputing.stackexchange.com/questions/29923/why-does-the-brk-instruction-set-the-b-flag
pub(crate) fn brk(s: &mut VmState) {
    println!("BRK {:04X}", s.reg.pc);
    s.push_word(s.reg.pc + 1);
    s.push(s.reg.p.bits());
    s.reg.pc = s.memory.fetch_word(IRQ);
    //p_set!(s.reg, B, true);
    p_set!(s.reg, I, true);
}

// http://www.6502.org/tutorials/6502opcodes.html#NOP
// http://www.6502.org/users/obelisk/6502/reference.html#NOP
pub(crate) fn nop(_s: &mut VmState) {}

#[cfg(test)]
mod tests {
    use crate::ops::misc::brk;
    use crate::{p_get, reg, Memory, RegBuilder, VmState, VmStateBuilder, _p, IRQ, P};
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn brk_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        s.memory.store_word(IRQ, 0x1234);
        assert!(!p_get!(s.reg, B));
        assert_eq!(0x0000, s.reg.pc);
        assert_eq!(0xff, s.reg.s);
        brk(&mut s);
        assert!(p_get!(s.reg, B));
        assert_eq!(0x1234, s.reg.pc);
        assert_eq!(0xfc, s.reg.s);
    }

    #[rstest]
    // cargo run -p r6502validation -- run-json '{ "name": "00 3f f7", "initial": { "pc": 35714, "s": 81, "a": 203, "x": 117, "y": 162, "p": 106, "ram": [ [35714, 0], [35715, 63], [35716, 247], [65534, 212], [65535, 37], [9684, 237]]}, "final": { "pc": 9684, "s": 78, "a": 203, "x": 117, "y": 162, "p": 110, "ram": [ [335, 122], [336, 132], [337, 139], [9684, 237], [35714, 0], [35715, 63], [35716, 247], [65534, 212], [65535, 37]]}, "cycles": [ [35714, 0, "read"], [35715, 63, "read"], [337, 139, "write"], [336, 132, "write"], [335, 122, "write"], [65534, 212, "read"], [65535, 37, "read"]] }'
    #[case(0x25d4, 110, 78, 0x8b82, 106, 81)]
    fn brk_scenarios(
        #[case] expected_pc: u16,
        #[case] expected_p: u8,
        #[case] expected_s: u8,
        #[case] pc: u16,
        #[case] p: u8,
        #[case] s: u8,
    ) -> Result<()> {
        let reg = RegBuilder::default().p(_p!(p)).s(s).build()?;
        let mut s = VmStateBuilder::default().reg(reg).build()?;
        s.reg.pc = pc + 1;
        brk(&mut s);
        assert_eq!(expected_pc, s.reg.pc);
        assert_eq!(_p!(expected_p), s.reg.p);
        assert_eq!(expected_s, s.reg.s);
        Ok(())
    }
}
