use crate::emulator::ops::helper::{is_neg, is_zero};
use crate::emulator::{Cpu, P};
use crate::{_p, p_set};

// http://www.6502.org/tutorials/6502opcodes.html#PHA
// http://www.6502.org/users/obelisk/6502/reference.html#PHA
pub fn pha(cpu: &mut Cpu) {
    cpu.push(cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#PHP
// http://www.6502.org/users/obelisk/6502/reference.html#PHP
pub fn php(cpu: &mut Cpu) {
    // https://www.nesdev.org/wiki/Status_flags
    // "B is 0 when pushed by interrupts (NMI and IRQ) and 1 when pushed by instructions (BRK and PHP)"
    cpu.push((cpu.reg.p | P::B).bits());
}

// http://www.6502.org/tutorials/6502opcodes.html#PLA
// http://www.6502.org/users/obelisk/6502/reference.html#PLA
pub fn pla(cpu: &mut Cpu) {
    let value = cpu.pull();
    cpu.reg.a = value;
    p_set!(cpu.reg, N, is_neg(value));
    p_set!(cpu.reg, Z, is_zero(value));
}

// http://www.6502.org/tutorials/6502opcodes.html#PLP
// http://www.6502.org/users/obelisk/6502/reference.html#PLP
pub fn plp(cpu: &mut Cpu) {
    // Retain ALWAYS_ONE and B
    let current_p = cpu.reg.p.bits();
    assert!((current_p & 0b00100000) == 0b00100000);
    let b_only = current_p & 0b00110000;

    // Without B
    let value = cpu.pull() & 0b11101111;

    cpu.reg.p = _p!(b_only | value);
}

#[cfg(test)]
mod tests {
    use crate::_p;
    use crate::emulator::ops::stack::{pha, php, pla, plp};
    use crate::emulator::{Bus, Cpu, P, STACK_BASE};
    use rstest::rstest;

    #[test]
    fn pha_basics() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x56;
        cpu.bus.store(STACK_BASE + 0x00ff, 0x34);
        assert_eq!(0xff, cpu.reg.sp);
        pha(&mut cpu);
        assert_eq!(0xfe, cpu.reg.sp);
        assert_eq!(0x56, cpu.reg.a);
        assert_eq!(P::default(), cpu.reg.p);
        assert_eq!(0x56, cpu.bus.load(STACK_BASE + 0x00ff));
    }

    #[test]
    fn pha_wraparound() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);

        for value in 0x00..=0xff {
            let current_s = 0xff - value;
            cpu.reg.a = value;
            cpu.bus.store(STACK_BASE + 0x00ff - value as u16, 0x00);
            assert_eq!(current_s, cpu.reg.sp);
            pha(&mut cpu);
            assert_eq!(current_s.wrapping_sub(1), cpu.reg.sp);
            assert_eq!(value, cpu.bus.load(STACK_BASE + 0x00ff - value as u16));
        }
    }

    #[test]
    fn php_basics() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);

        cpu.reg.p = P::N | P::ALWAYS_ONE | P::D | P::Z;
        php(&mut cpu);

        cpu.reg.p = P::V | P::ALWAYS_ONE | P::C;
        php(&mut cpu);

        cpu.reg.p = P::ALWAYS_ONE;

        plp(&mut cpu);
        assert_eq!(P::V | P::ALWAYS_ONE | P::C, cpu.reg.p);

        plp(&mut cpu);
        assert_eq!(P::N | P::ALWAYS_ONE | P::D | P::Z, cpu.reg.p);
    }

    #[rstest]
    // cargo run -- validate-json '{ "name": "08 60 be", "initial": { "pc": 12161, "s": 38, "a": 135, "x": 106, "y": 180, "p": 43, "ram": [ [12161, 8], [12162, 96], [12163, 190]]}, "final": { "pc": 12162, "s": 37, "a": 135, "x": 106, "y": 180, "p": 43, "ram": [ [294, 59], [12161, 8], [12162, 96], [12163, 190]]}, "cycles": [ [12161, 8, "read"], [12162, 96, "read"], [294, 59, "write"]] }'
    #[case(37, 0x0126, 59, 38, 43)]
    fn php_scenarios(
        #[case] expected_s: u8,
        #[case] expected_addr: u16,
        #[case] expected_value: u8,
        #[case] sp: u8,
        #[case] p: u8,
    ) {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);

        cpu.reg.sp = sp;
        cpu.reg.p = _p!(p);
        php(&mut cpu);
        assert_eq!(expected_s, cpu.reg.sp);
        assert_eq!(expected_value, cpu.bus.load(expected_addr));
    }

    #[test]
    fn pla_basics() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);

        cpu.reg.a = 0x00;
        pha(&mut cpu);

        cpu.reg.a = 0xf1;
        pha(&mut cpu);

        cpu.reg.a = 0x45;
        pha(&mut cpu);

        cpu.reg.a = 0x11;
        cpu.reg.p = P::empty();
        assert_eq!(0x11, cpu.reg.a);

        pla(&mut cpu);
        assert_eq!(0x45, cpu.reg.a);
        assert_eq!(P::empty(), cpu.reg.p);

        pla(&mut cpu);
        assert_eq!(0xf1, cpu.reg.a);
        assert_eq!(P::N, cpu.reg.p);

        pla(&mut cpu);
        assert_eq!(0x00, cpu.reg.a);
        assert_eq!(P::Z, cpu.reg.p);
    }

    #[test]
    fn p_flag_basics() {
        fn do_test(expected_p: u8, cpu: &mut Cpu, start: u16, value: u8) {
            cpu.bus.store(start + 1, value); // the test value
            cpu.reg.p = _p!(0b00110000);
            cpu.reg.pc = start;
            _ = cpu.step_no_spin(); // LDA #value
            assert_eq!(start + 2, cpu.reg.pc);
            assert_eq!(value, cpu.reg.a);
            _ = cpu.step_no_spin(); // PHA
            assert_eq!(start + 3, cpu.reg.pc);
            assert_eq!(value, cpu.bus.load(STACK_BASE + cpu.reg.sp as u16 + 1));
            _ = cpu.step_no_spin(); // PLP
            assert_eq!(_p!(expected_p), cpu.reg.p);
        }
        const START: u16 = 0x1000;

        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);

        bus.store(START, 0xa9); // LDA_IMM
        bus.store(START + 2, 0x48); // PHA
        bus.store(START + 3, 0x28); // PLP

        do_test(0b11111111, &mut cpu, START, 0b11111111);
        do_test(0b00110000, &mut cpu, START, 0b00000000);
        do_test(0b00110000, &mut cpu, START, 0b00110000);
        do_test(0b00110001, &mut cpu, START, 0b00110001);
        do_test(0b10110001, &mut cpu, START, 0b10110001);
    }
}
