use crate::emulator::Cpu;
use crate::emulator::ops::helper::set_flags_on_value;

// http://www.6502.org/tutorials/6502opcodes.html#LDA
// http://www.6502.org/users/obelisk/6502/reference.html#LDA
pub fn lda(cpu: &mut Cpu, operand: u8) {
    cpu.reg.a = operand;
    set_flags_on_value(cpu, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#LDX
// http://www.6502.org/users/obelisk/6502/reference.html#LDX
pub fn ldx(cpu: &mut Cpu, operand: u8) {
    cpu.reg.x = operand;
    set_flags_on_value(cpu, operand);
}

// http://www.6502.org/tutorials/6502opcodes.html#LDY
// http://www.6502.org/users/obelisk/6502/reference.html#LDY
pub fn ldy(cpu: &mut Cpu, operand: u8) {
    cpu.reg.y = operand;
    set_flags_on_value(cpu, operand);
}

#[cfg(test)]
mod tests {
    use crate::emulator::ops::load::{lda, ldx, ldy};
    use crate::emulator::{Bus, Cpu};
    use crate::{InterruptChannel, P, p};
    use rstest::rstest;

    #[rstest]
    // LDA #0
    #[case(p!(Z), 0x00)]
    // LDA #1
    #[case(p!(), 0x01)]
    // LDA #$255
    #[case(p!(N), 0xff)]
    fn lda_basics(#[case] expected_p: P, #[case] operand: u8) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = 0xff;
        lda(&mut cpu, operand);
        assert_eq!(operand, cpu.reg.a);
        assert_eq!(expected_p, cpu.reg.p);
    }

    #[rstest]
    // LDX #0
    #[case(p!(Z), 0x00)]
    // LDX #1
    #[case(p!(), 0x01)]
    // LDX #$255
    #[case(p!(N), 0xff)]
    fn ldx_basics(#[case] expected_p: P, #[case] operand: u8) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.x = 0xff;
        ldx(&mut cpu, operand);
        assert_eq!(operand, cpu.reg.x);
        assert_eq!(expected_p, cpu.reg.p);
    }

    #[rstest]
    // LDY #0
    #[case(p!(Z), 0x00)]
    // LDY #1
    #[case(p!(), 0x01)]
    // LDY #$255
    #[case(p!(N), 0xff)]
    fn ldy_basics(#[case] expected_p: P, #[case] operand: u8) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.y = 0xff;
        ldy(&mut cpu, operand);
        assert_eq!(operand, cpu.reg.y);
        assert_eq!(expected_p, cpu.reg.p);
    }
}
