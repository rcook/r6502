use crate::emulator::Cpu;
use crate::emulator::ops::helper::set_flags_on_value;

// http://www.6502.org/tutorials/6502opcodes.html#DEC
// http://www.6502.org/users/obelisk/6502/reference.html#DEC
pub fn dec(cpu: &mut Cpu, addr: u16) {
    let result = cpu.bus.load(addr).wrapping_sub(1);
    cpu.bus.store(addr, result);
    set_flags_on_value(cpu, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#INC
// http://www.6502.org/users/obelisk/6502/reference.html#INC
pub fn inc(cpu: &mut Cpu, addr: u16) {
    let result = cpu.bus.load(addr).wrapping_add(1);
    cpu.bus.store(addr, result);
    set_flags_on_value(cpu, result);
}

// http://www.6502.org/tutorials/6502opcodes.html#STA
// http://www.6502.org/users/obelisk/6502/reference.html#STA
pub fn sta(cpu: &mut Cpu, addr: u16) {
    cpu.bus.store(addr, cpu.reg.a);
}

// http://www.6502.org/tutorials/6502opcodes.html#STX
// http://www.6502.org/users/obelisk/6502/reference.html#STX
pub fn stx(cpu: &mut Cpu, addr: u16) {
    cpu.bus.store(addr, cpu.reg.x);
}

// http://www.6502.org/tutorials/6502opcodes.html#STY
// http://www.6502.org/users/obelisk/6502/reference.html#STY
pub fn sty(cpu: &mut Cpu, addr: u16) {
    cpu.bus.store(addr, cpu.reg.y);
}
