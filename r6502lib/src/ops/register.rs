use super::helper::set_flags_on_value;
use crate::Cpu;

// http://www.6502.org/tutorials/6502opcodes.html#DEX
// http://www.6502.org/users/obelisk/6502/reference.html#DEX
pub(crate) fn dex(state: &mut Cpu) {
    let value = state.reg.x.wrapping_sub(1);
    state.reg.x = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#DEY
// http://www.6502.org/users/obelisk/6502/reference.html#DEY
pub(crate) fn dey(state: &mut Cpu) {
    let value = state.reg.y.wrapping_sub(1);
    state.reg.y = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#INX
// http://www.6502.org/users/obelisk/6502/reference.html#INX
pub(crate) fn inx(state: &mut Cpu) {
    let value = state.reg.x.wrapping_add(1);
    state.reg.x = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#INY
// http://www.6502.org/users/obelisk/6502/reference.html#INY
pub(crate) fn iny(state: &mut Cpu) {
    let value = state.reg.y.wrapping_add(1);
    state.reg.y = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TAX
// http://www.6502.org/users/obelisk/6502/reference.html#TAX
pub(crate) fn tax(state: &mut Cpu) {
    let value = state.reg.a;
    state.reg.x = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TAY
// http://www.6502.org/users/obelisk/6502/reference.html#TAY
pub(crate) fn tay(state: &mut Cpu) {
    let value = state.reg.a;
    state.reg.y = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TSX
// http://www.6502.org/users/obelisk/6502/reference.html#TSX
pub(crate) fn tsx(state: &mut Cpu) {
    let value = state.reg.sp;
    state.reg.x = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TXA
// http://www.6502.org/users/obelisk/6502/reference.html#TXA
pub(crate) fn txa(state: &mut Cpu) {
    let value = state.reg.x;
    state.reg.a = value;
    set_flags_on_value(state, value);
}

// http://www.6502.org/tutorials/6502opcodes.html#TXS
// http://www.6502.org/users/obelisk/6502/reference.html#TXS
pub(crate) fn txs(state: &mut Cpu) {
    let value = state.reg.x;
    state.reg.sp = value;
}

// http://www.6502.org/tutorials/6502opcodes.html#TYA
// http://www.6502.org/users/obelisk/6502/reference.html#TYA
pub(crate) fn tya(state: &mut Cpu) {
    let value = state.reg.y;
    state.reg.a = value;
    set_flags_on_value(state, value);
}

#[cfg(test)]
mod tests {
    use crate::ops::register::{tax, tay, txa, tya};
    use crate::{Cpu, Memory};

    #[test]
    fn tax_basics() {
        let memory = Memory::default();
        let mut cpu = Cpu::new(memory.view(), None);
        cpu.reg.a = 0x22;
        cpu.reg.x = 0x00;
        tax(&mut cpu);
        assert_eq!(0x22, cpu.reg.x);
    }

    #[test]
    fn tay_basics() {
        let memory = Memory::default();
        let mut cpu = Cpu::new(memory.view(), None);
        cpu.reg.a = 0x22;
        cpu.reg.y = 0x00;
        tay(&mut cpu);
        assert_eq!(0x22, cpu.reg.y);
    }

    #[test]
    fn txa_basics() {
        let memory = Memory::default();
        let mut cpu = Cpu::new(memory.view(), None);
        cpu.reg.a = 0x00;
        cpu.reg.x = 0x22;
        txa(&mut cpu);
        assert_eq!(0x22, cpu.reg.a);
    }

    #[test]
    fn tya_basics() {
        let memory = Memory::default();
        let mut cpu = Cpu::new(memory.view(), None);
        cpu.reg.a = 0x00;
        cpu.reg.y = 0x22;
        tya(&mut cpu);
        assert_eq!(0x22, cpu.reg.a);
    }
}
