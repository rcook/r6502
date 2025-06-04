use crate::ops::helper::{is_neg, is_zero};
use crate::{p_set, VmState, _p, P};

fn set_p(s: &mut VmState, value: u8) {
    s.reg.p = _p!(value | 0b00110000)
}

// http://www.6502.org/tutorials/6502opcodes.html#PHA
// http://www.6502.org/users/obelisk/6502/reference.html#PHA
pub(crate) fn pha(s: &mut VmState) {
    s.push(s.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#PHP
// http://www.6502.org/users/obelisk/6502/reference.html#PHP
pub(crate) fn php(s: &mut VmState) {
    s.push((s.reg.p | P::B).bits());
}

// http://www.6502.org/tutorials/6502opcodes.html#PLA
// http://www.6502.org/users/obelisk/6502/reference.html#PLA
pub(crate) fn pla(s: &mut VmState) {
    let value = s.pull();
    s.reg.a = value;
    p_set!(s.reg, N, is_neg(value));
    p_set!(s.reg, Z, is_zero(value));
}

// http://www.6502.org/tutorials/6502opcodes.html#PLP
// http://www.6502.org/users/obelisk/6502/reference.html#PLP
pub(crate) fn plp(s: &mut VmState) {
    let value = s.pull();
    set_p(s, value);
}

#[cfg(test)]
mod tests {
    use crate::ops::stack::{pha, php, pla, plp};
    use crate::{reg, Memory, Vm, VmBuilder, VmState, VmStateBuilder, _p, P, STACK_BASE};
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn pha_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };
        s.reg.a = 0x56;
        s.memory[STACK_BASE + 0x00ff] = 0x34;
        assert_eq!(0xff, s.reg.s);
        pha(&mut s);
        assert_eq!(0xfe, s.reg.s);
        assert_eq!(0x56, s.reg.a);
        assert_eq!(P::default(), s.reg.p);
        assert_eq!(0x56, s.memory[STACK_BASE + 0x00ff])
    }

    #[test]
    fn pha_wraparound() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        for value in 0x00..=0xff {
            let current_s = 0xff - value;
            s.reg.a = value;
            s.memory[STACK_BASE + 0x00ff - value as u16] = 0x00;
            assert_eq!(current_s, s.reg.s);
            pha(&mut s);
            assert_eq!(current_s.wrapping_sub(1), s.reg.s);
            assert_eq!(value, s.memory[STACK_BASE + 0x00ff - value as u16])
        }
    }

    #[test]
    fn php_basics() -> Result<()> {
        let mut s = VmStateBuilder::default().build()?;

        s.reg.p = P::N | P::D | P::Z;
        php(&mut s);

        s.reg.p = P::V | P::C;
        php(&mut s);

        s.reg.p = P::empty();

        plp(&mut s);
        assert_eq!(P::V | P::ALWAYS_ONE | P::B | P::C, s.reg.p);

        plp(&mut s);
        assert_eq!(P::N | P::ALWAYS_ONE | P::B | P::D | P::Z, s.reg.p);

        Ok(())
    }

    #[rstest]
    // cargo run -p r6502validation -- run-json '{ "name": "08 60 be", "initial": { "pc": 12161, "s": 38, "a": 135, "x": 106, "y": 180, "p": 43, "ram": [ [12161, 8], [12162, 96], [12163, 190]]}, "final": { "pc": 12162, "s": 37, "a": 135, "x": 106, "y": 180, "p": 43, "ram": [ [294, 59], [12161, 8], [12162, 96], [12163, 190]]}, "cycles": [ [12161, 8, "read"], [12162, 96, "read"], [294, 59, "write"]] }'
    #[case(37, 0x0126, 59, 38, 43)]
    fn php_scenarios(
        #[case] expected_s: u8,
        #[case] expected_addr: u16,
        #[case] expected_value: u8,
        #[case] s: u8,
        #[case] p: u8,
    ) {
        let mut vm_state = VmState::default();
        vm_state.reg.s = s;
        vm_state.reg.p = _p!(p);
        php(&mut vm_state);
        assert_eq!(expected_s, vm_state.reg.s);
        assert_eq!(expected_value, vm_state.memory[expected_addr]);
    }

    #[test]
    fn pla_basics() {
        let mut s = VmState {
            reg: reg!(0xff, 0x0000),
            memory: Memory::new(),
        };

        s.reg.a = 0x00;
        pha(&mut s);

        s.reg.a = 0xf1;
        pha(&mut s);

        s.reg.a = 0x45;
        pha(&mut s);

        s.reg.a = 0x11;
        s.reg.p = P::empty();
        assert_eq!(0x11, s.reg.a);

        pla(&mut s);
        assert_eq!(0x45, s.reg.a);
        assert_eq!(P::empty(), s.reg.p);

        pla(&mut s);
        assert_eq!(0xf1, s.reg.a);
        assert_eq!(P::N, s.reg.p);

        pla(&mut s);
        assert_eq!(0x00, s.reg.a);
        assert_eq!(P::Z, s.reg.p);
    }

    #[test]
    fn p_flag_basics() -> Result<()> {
        fn do_test(expected_p: u8, vm: &mut Vm, start: u16, value: u8) {
            vm.s.memory[start + 1] = value; // the test value
            vm.s.reg.p = _p!(0b00110000);
            vm.s.reg.pc = start;
            _ = vm.step();
            vm.s.reg.pc = start + 2;
            _ = vm.step();
            vm.s.reg.pc = start + 3;
            _ = vm.step();
            assert_eq!(_p!(expected_p), vm.s.reg.p);
        }
        const START: u16 = 0x1000;

        let mut vm = VmBuilder::default().build()?;

        vm.s.memory[START] = 0xa9; // LDA_IMM
        vm.s.memory[START + 2] = 0x48; // PHA
        vm.s.memory[START + 3] = 0x28; // PLP

        do_test(0b11111111, &mut vm, START, 0b11111111);
        do_test(0b00110000, &mut vm, START, 0b00000000);
        do_test(0b00110000, &mut vm, START, 0b00110000);
        do_test(0b00110001, &mut vm, START, 0b00110001);

        Ok(())
    }
}
