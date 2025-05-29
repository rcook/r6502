use crate::{Cpu, Cycles, Instruction, Monitor, VmState};

#[allow(unused)]
pub(crate) fn step(monitor: &impl Monitor, cpu: &Cpu, s: &mut VmState) -> Cycles {
    monitor.on_before_fetch(&s.reg);
    let instruction = Instruction::fetch(cpu, s);
    monitor.on_before_execute(&s.reg, &instruction);
    let cycles = instruction.execute(s);
    monitor.on_after_execute(&s.reg, &instruction, cycles);
    cycles
}

#[cfg(test)]
mod tests {
    use crate::{p, step, Cpu, DummyMonitor, Memory, Opcode, Reg, VmState};

    #[test]
    fn no_operand() {
        let mut s = VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        };
        s.memory[0x0000] = Opcode::Nop as u8;
        let cycles = step(&DummyMonitor, &Cpu::make_6502(), &mut s);
        assert_eq!(2, cycles);
        assert_eq!(0x12, s.reg.a);
        assert_eq!(p!(), s.reg.p);
        assert_eq!(0x0001, s.reg.pc)
    }

    #[test]
    fn byte0() {
        let mut s = VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        };
        s.memory[0x0000] = Opcode::AdcImm as u8;
        s.memory[0x0001] = 0x34;
        let cycles = step(&DummyMonitor, &Cpu::make_6502(), &mut s);
        assert_eq!(2, cycles);
        assert_eq!(0x46, s.reg.a);
        assert_eq!(p!(), s.reg.p);
        assert_eq!(0x0002, s.reg.pc)
    }

    #[test]
    fn byte1() {
        let mut s = VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        };
        s.memory[0x0000] = Opcode::AdcZp as u8;
        s.memory[0x0001] = 0x34;
        s.memory[0x0034] = 0x56;
        let cycles = step(&DummyMonitor, &Cpu::make_6502(), &mut s);
        assert_eq!(3, cycles);
        assert_eq!(0x68, s.reg.a);
        assert_eq!(p!(), s.reg.p);
        assert_eq!(0x0002, s.reg.pc)
    }

    #[test]
    fn word0() {
        let mut s = VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        };
        s.memory[0x0000] = Opcode::JmpAbs as u8;
        s.memory[0x0001] = 0x00;
        s.memory[0x0002] = 0x10;
        let cycles = step(&DummyMonitor, &Cpu::make_6502(), &mut s);
        assert_eq!(3, cycles);
        assert_eq!(0x12, s.reg.a);
        assert_eq!(p!(), s.reg.p);
        assert_eq!(0x1000, s.reg.pc)
    }

    #[test]
    fn word1() {
        let mut s = VmState {
            reg: Reg {
                a: 0x25,
                ..Default::default()
            },
            memory: Memory::new(),
        };
        s.memory[0x0000] = Opcode::AdcAbs as u8;
        s.memory[0x0001] = 0x12;
        s.memory[0x0002] = 0x34;
        s.memory[0x3412] = 0x13;
        let cycles = step(&DummyMonitor, &Cpu::make_6502(), &mut s);
        assert_eq!(4, cycles);
        assert_eq!(0x38, s.reg.a);
        assert_eq!(p!(), s.reg.p);
        assert_eq!(0x0003, s.reg.pc)
    }
}
