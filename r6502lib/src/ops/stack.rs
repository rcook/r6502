use crate::ops::helper::{is_neg, is_zero};
use crate::{p_set, CpuState, _p, P};

// http://www.6502.org/tutorials/6502opcodes.html#PHA
// http://www.6502.org/users/obelisk/6502/reference.html#PHA
pub(crate) fn pha(state: &mut CpuState) {
    state.push(state.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#PHP
// http://www.6502.org/users/obelisk/6502/reference.html#PHP
pub(crate) fn php(state: &mut CpuState) {
    // https://www.nesdev.org/wiki/Status_flags
    // "B is 0 when pushed by interrupts (NMI and IRQ) and 1 when pushed by instructions (BRK and PHP)"
    state.push((state.reg.p | P::B).bits());
}

// http://www.6502.org/tutorials/6502opcodes.html#PLA
// http://www.6502.org/users/obelisk/6502/reference.html#PLA
pub(crate) fn pla(state: &mut CpuState) {
    let value = state.pull();
    state.reg.a = value;
    p_set!(state.reg, N, is_neg(value));
    p_set!(state.reg, Z, is_zero(value));
}

// http://www.6502.org/tutorials/6502opcodes.html#PLP
// http://www.6502.org/users/obelisk/6502/reference.html#PLP
pub(crate) fn plp(state: &mut CpuState) {
    // Retain ALWAYS_ONE and B
    let current_p = state.reg.p.bits();
    assert!((current_p & 0b00100000) == 0b00100000);
    let b_only = current_p & 0b00110000;

    // Without B
    let value = state.pull() & 0b11101111;

    state.reg.p = _p!(b_only | value);
}

#[cfg(test)]
mod tests {
    use crate::ops::stack::{pha, php, pla, plp};
    use crate::{Cpu, CpuState, DummyMonitor, Memory, Reg, _p, P, STACK_BASE};
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn pha_basics() {
        let memory = Memory::default();
        let mut state = CpuState::new(Reg::default(), memory.view());
        state.reg.a = 0x56;
        state.memory.store(STACK_BASE + 0x00ff, 0x34);
        assert_eq!(0xff, state.reg.sp);
        pha(&mut state);
        assert_eq!(0xfe, state.reg.sp);
        assert_eq!(0x56, state.reg.a);
        assert_eq!(P::default(), state.reg.p);
        assert_eq!(0x56, state.memory.load(STACK_BASE + 0x00ff))
    }

    #[test]
    fn pha_wraparound() {
        let memory = Memory::default();
        let mut state = CpuState::new(Reg::default(), memory.view());

        for value in 0x00..=0xff {
            let current_s = 0xff - value;
            state.reg.a = value;
            state.memory.store(STACK_BASE + 0x00ff - value as u16, 0x00);
            assert_eq!(current_s, state.reg.sp);
            pha(&mut state);
            assert_eq!(current_s.wrapping_sub(1), state.reg.sp);
            assert_eq!(value, state.memory.load(STACK_BASE + 0x00ff - value as u16))
        }
    }

    #[test]
    fn php_basics() -> Result<()> {
        let memory = Memory::default();
        let mut state = CpuState::new(Reg::default(), memory.view());

        state.reg.p = P::N | P::ALWAYS_ONE | P::D | P::Z;
        php(&mut state);

        state.reg.p = P::V | P::ALWAYS_ONE | P::C;
        php(&mut state);

        state.reg.p = P::ALWAYS_ONE;

        plp(&mut state);
        assert_eq!(P::V | P::ALWAYS_ONE | P::C, state.reg.p);

        plp(&mut state);
        assert_eq!(P::N | P::ALWAYS_ONE | P::D | P::Z, state.reg.p);

        Ok(())
    }

    #[rstest]
    // cargo run -p r6502validation -- run-json '{ "name": "08 60 be", "initial": { "pc": 12161, "s": 38, "a": 135, "x": 106, "y": 180, "p": 43, "ram": [ [12161, 8], [12162, 96], [12163, 190]]}, "final": { "pc": 12162, "s": 37, "a": 135, "x": 106, "y": 180, "p": 43, "ram": [ [294, 59], [12161, 8], [12162, 96], [12163, 190]]}, "cycles": [ [12161, 8, "read"], [12162, 96, "read"], [294, 59, "write"]] }'
    #[case(37, 0x0126, 59, 38, 43)]
    fn php_scenarios(
        #[case] expected_s: u8,
        #[case] expected_addr: u16,
        #[case] expected_value: u8,
        #[case] sp: u8,
        #[case] p: u8,
    ) {
        let memory = Memory::default();
        let mut state = CpuState::new(Reg::default(), memory.view());
        state.reg.sp = sp;
        state.reg.p = _p!(p);
        php(&mut state);
        assert_eq!(expected_s, state.reg.sp);
        assert_eq!(expected_value, state.memory.load(expected_addr));
    }

    #[test]
    fn pla_basics() {
        let memory = Memory::default();
        let mut state = CpuState::new(Reg::default(), memory.view());

        state.reg.a = 0x00;
        pha(&mut state);

        state.reg.a = 0xf1;
        pha(&mut state);

        state.reg.a = 0x45;
        pha(&mut state);

        state.reg.a = 0x11;
        state.reg.p = P::empty();
        assert_eq!(0x11, state.reg.a);

        pla(&mut state);
        assert_eq!(0x45, state.reg.a);
        assert_eq!(P::empty(), state.reg.p);

        pla(&mut state);
        assert_eq!(0xf1, state.reg.a);
        assert_eq!(P::N, state.reg.p);

        pla(&mut state);
        assert_eq!(0x00, state.reg.a);
        assert_eq!(P::Z, state.reg.p);
    }

    #[test]
    fn p_flag_basics() -> Result<()> {
        fn do_test(expected_p: u8, vm: &mut Cpu, start: u16, value: u8) {
            vm.s.memory.store(start + 1, value); // the test value
            vm.s.reg.p = _p!(0b00110000);
            vm.s.reg.pc = start;
            _ = vm.step(); // LDA #value
            assert_eq!(start + 2, vm.s.reg.pc);
            assert_eq!(value, vm.s.reg.a);
            _ = vm.step(); // PHA
            assert_eq!(start + 3, vm.s.reg.pc);
            assert_eq!(value, vm.s.memory.load(STACK_BASE + vm.s.reg.sp as u16 + 1));
            _ = vm.step(); // PLP
            assert_eq!(_p!(expected_p), vm.s.reg.p);
        }
        const START: u16 = 0x1000;

        let memory = Memory::default();
        let mut cpu = Cpu::new(
            Box::new(DummyMonitor),
            CpuState::new(Reg::default(), memory.view()),
        );

        memory.store(START, 0xa9); // LDA_IMM
        memory.store(START + 2, 0x48); // PHA
        memory.store(START + 3, 0x28); // PLP

        do_test(0b11111111, &mut cpu, START, 0b11111111);
        do_test(0b00110000, &mut cpu, START, 0b00000000);
        do_test(0b00110000, &mut cpu, START, 0b00110000);
        do_test(0b00110001, &mut cpu, START, 0b00110001);
        do_test(0b10110001, &mut cpu, START, 0b10110001);

        Ok(())
    }
}
