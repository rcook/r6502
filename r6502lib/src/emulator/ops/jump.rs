use crate::emulator::util::{make_word, split_word};
use crate::emulator::{Cpu, P};
use crate::p_set;

// http://www.6502.org/tutorials/6502opcodes.html#JMP
// http://www.6502.org/users/obelisk/6502/reference.html#JMP
pub const fn jmp(cpu: &mut Cpu, operand: u16) {
    cpu.reg.pc = operand;
}

// http://www.6502.org/tutorials/6502opcodes.html#JSR
// http://www.6502.org/users/obelisk/6502/reference.html#JSR
pub fn jsr(cpu: &mut Cpu, addr: u16) {
    // We can look back at the bytes immediately before PC and the
    // target address should be exactly the same as the argument
    // to this function.
    let hi_addr = cpu.reg.pc.wrapping_sub(1);
    let lo_addr = cpu.reg.pc.wrapping_sub(2);
    let effective_addr = make_word(cpu.bus.load(hi_addr), cpu.bus.load(lo_addr));
    assert_eq!(addr, effective_addr);

    // The real JSR instruction starts to push the return address onto
    // the stack before fetching its argument. If the JSR instruction
    // happens to be located at the top of the stack in memory, this
    // will result in it fetching a combination of the return address
    // and the operand. To fully emulate JSR, we must use this address
    // even if it's garbage.
    let (return_hi, return_lo) = split_word(cpu.reg.pc.wrapping_sub(1));
    cpu.push(return_hi);
    let effective_addr = make_word(cpu.bus.load(hi_addr), cpu.bus.load(lo_addr));
    cpu.push(return_lo);
    cpu.reg.pc = effective_addr;
}

// http://www.6502.org/tutorials/6502opcodes.html#RTI
// http://www.6502.org/users/obelisk/6502/reference.html#RTI
pub fn rti(cpu: &mut Cpu) {
    cpu.reg.p = P::from_bits(cpu.pull()).expect("Must succeed");
    p_set!(cpu.reg, ALWAYS_ONE, true);
    p_set!(cpu.reg, B, false);
    let return_addr = cpu.pull_word();
    cpu.reg.pc = return_addr;
}

// http://www.6502.org/tutorials/6502opcodes.html#RTS
// http://www.6502.org/users/obelisk/6502/reference.html#RTS
pub fn rts(cpu: &mut Cpu) {
    let return_addr = cpu.pull_word().wrapping_add(1);
    cpu.reg.pc = return_addr;
}

#[cfg(test)]
mod tests {
    use crate::_p;
    use crate::emulator::ops::{jmp, jsr, rti};
    use crate::emulator::util::split_word;
    use crate::emulator::{Bus, Cpu, IrqChannel};
    use rstest::rstest;

    #[rstest]
    #[case(0x12, 0x1000, 0x12, 0x0000, 0x1000)]
    fn jmp_basics(
        #[case] expected_a: u8,
        #[case] expected_pc: u16,
        #[case] a: u8,
        #[case] pc: u16,
        #[case] operand: u16,
    ) {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.a = a;
        cpu.reg.pc = pc;
        jmp(&mut cpu, operand);
        assert_eq!(expected_a, cpu.reg.a);
        assert_eq!(expected_pc, cpu.reg.pc);
    }

    #[test]
    fn jsr_basics() {
        const TARGET_ADDR: u16 = 0x1234;

        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        let (target_hi, target_lo) = split_word(TARGET_ADDR);
        bus.store(0x1000, 0x20);
        bus.store(0x1001, target_lo);
        bus.store(0x1002, target_hi);

        // TBD: Restructure VM so that instruction and operand decoding
        // is taken account of properly. For now, we'll work around this
        // behaviour in the implementation of JSR above.
        cpu.reg.pc = 0x1003; // Opcode and operand have been decoded
        jsr(&mut cpu, TARGET_ADDR);
        assert_eq!(0x1002, cpu.peek_word());
        assert_eq!(TARGET_ADDR, cpu.reg.pc);
    }

    // Scenario: 20 55 13
    // An interesting case in which the JSR instruction smashes its
    // own argument: its push operations must happen _before_ the
    // operands are fetched, so that JSR to address at the top of the
    // stack will have bizarre behaviour
    #[test]
    fn jsr_smashing_stack() {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);

        cpu.reg.pc = 0x017b;
        cpu.reg.sp = 0x7d;
        cpu.reg.a = 0x9e; // Probably irrelevant
        cpu.reg.x = 0x89; // Probably irrelevant
        cpu.reg.y = 0x34; // Probably irrelevant
        cpu.reg.p = _p!(0b1110_0110); // Probably irrelevant
        bus.store(0x0155, 0xad);
        bus.store(0x017b, 0x20); // JSR abs
        bus.store(0x017c, 0x55);
        bus.store(0x017d, 0x13);
        cpu.step_no_spin();
        assert_eq!(0x0155, cpu.reg.pc);
        assert_eq!(0x7b, cpu.reg.sp);
        assert_eq!(0x9e, cpu.reg.a);
        assert_eq!(0x89, cpu.reg.x);
        assert_eq!(0x34, cpu.reg.y);
        assert_eq!(_p!(0b1110_0110), cpu.reg.p);
        assert_eq!(0xad, bus.load(0x0155));
        assert_eq!(0x20, bus.load(0x017b));
        assert_eq!(0x7d, bus.load(0x017c));
        assert_eq!(0x01, bus.load(0x017d));
    }

    #[test]
    // cargo run -- validate-json '{ "name": "40 9c 2c", "initial": { "pc": 34673, "s": 110, "a": 162, "x": 129, "y": 126, "p": 99, "ram": [ [34673, 64], [34674, 156], [34675, 44], [366, 152], [367, 156], [368, 170], [369, 101], [26026, 14]]}, "final": { "pc": 26026, "s": 113, "a": 162, "x": 129, "y": 126, "p": 172, "ram": [ [366, 152], [367, 156], [368, 170], [369, 101], [26026, 14], [34673, 64], [34674, 156], [34675, 44]]}, "cycles": [ [34673, 64, "read"], [34674, 156, "read"], [366, 152, "read"], [367, 156, "read"], [368, 170, "read"], [369, 101, "read"]] }'
    fn rti_scenario() {
        const INITIAL_SP: u8 = 0x6e;

        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.p = _p!(0x63);
        cpu.reg.sp = INITIAL_SP;
        bus.store(0x0100 + u16::from(INITIAL_SP), 0x98);
        bus.store(0x0100 + u16::from(INITIAL_SP) + 1, 0x9c); // P
        bus.store(0x0100 + u16::from(INITIAL_SP) + 2, 0xaa); // lo(return_attr)
        bus.store(0x0100 + u16::from(INITIAL_SP) + 3, 0x65); // hi(return_attr)
        cpu.reg.pc = 0x8771 + 1;
        rti(&mut cpu);
        assert_eq!(0x65aa, cpu.reg.pc);
        assert_eq!(_p!(0xac), cpu.reg.p); // P & 0b11101111
        assert_eq!(INITIAL_SP + 3, cpu.reg.sp);
    }
}
