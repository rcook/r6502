use crate::emulator::Cpu;
use crate::emulator::ops::helper::set_flags_on_value;

// http://www.6502.org/tutorials/6502opcodes.html#DEX
// http://www.6502.org/users/obelisk/6502/reference.html#DEX
pub fn dex(cpu: &mut Cpu) {
    let value = cpu.reg.x.wrapping_sub(1);
    cpu.reg.x = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#DEY
// http://www.6502.org/users/obelisk/6502/reference.html#DEY
pub fn dey(cpu: &mut Cpu) {
    let value = cpu.reg.y.wrapping_sub(1);
    cpu.reg.y = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#INX
// http://www.6502.org/users/obelisk/6502/reference.html#INX
pub fn inx(cpu: &mut Cpu) {
    let value = cpu.reg.x.wrapping_add(1);
    cpu.reg.x = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#INY
// http://www.6502.org/users/obelisk/6502/reference.html#INY
pub fn iny(cpu: &mut Cpu) {
    let value = cpu.reg.y.wrapping_add(1);
    cpu.reg.y = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TAX
// http://www.6502.org/users/obelisk/6502/reference.html#TAX
pub fn tax(cpu: &mut Cpu) {
    let value = cpu.reg.a;
    cpu.reg.x = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TAY
// http://www.6502.org/users/obelisk/6502/reference.html#TAY
pub fn tay(cpu: &mut Cpu) {
    let value = cpu.reg.a;
    cpu.reg.y = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TSX
// http://www.6502.org/users/obelisk/6502/reference.html#TSX
pub fn tsx(cpu: &mut Cpu) {
    let value = cpu.reg.sp;
    cpu.reg.x = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TXA
// http://www.6502.org/users/obelisk/6502/reference.html#TXA
pub fn txa(cpu: &mut Cpu) {
    let value = cpu.reg.x;
    cpu.reg.a = value;
    set_flags_on_value(cpu, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TXS
// http://www.6502.org/users/obelisk/6502/reference.html#TXS
pub const fn txs(cpu: &mut Cpu) {
    let value = cpu.reg.x;
    cpu.reg.sp = value;
}

// http://www.6502.org/tutorials/6502opcodes.html#TYA
// http://www.6502.org/users/obelisk/6502/reference.html#TYA
pub fn tya(cpu: &mut Cpu) {
    let value = cpu.reg.y;
    cpu.reg.a = value;
    set_flags_on_value(cpu, value);
}

#[cfg(test)]
mod tests {
    use crate::InterruptChannel;
    use crate::emulator::ops::register::{tax, tay, txa, tya};
    use crate::emulator::{Bus, Cpu};

    #[test]
    fn tax_basics() {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = 0x22;
        cpu.reg.x = 0x00;
        tax(&mut cpu);
        assert_eq!(0x22, cpu.reg.x);
    }

    #[test]
    fn tay_basics() {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = 0x22;
        cpu.reg.y = 0x00;
        tay(&mut cpu);
        assert_eq!(0x22, cpu.reg.y);
    }

    #[test]
    fn txa_basics() {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = 0x00;
        cpu.reg.x = 0x22;
        txa(&mut cpu);
        assert_eq!(0x22, cpu.reg.a);
    }

    #[test]
    fn tya_basics() {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = 0x00;
        cpu.reg.y = 0x22;
        tya(&mut cpu);
        assert_eq!(0x22, cpu.reg.a);
    }
}
