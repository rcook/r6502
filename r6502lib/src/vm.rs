use crate::{Cpu, Cycles, Instruction, InstructionInfo, Monitor, VmState};

#[allow(unused)]
pub(crate) fn step(monitor: &impl Monitor, cpu: &Cpu, s: &mut VmState) -> Cycles {
    monitor.on_before_fetch(&s.reg);
    let instruction = Instruction::fetch(cpu, s);
    let instruction_info = InstructionInfo::from_instruction(&instruction);
    monitor.on_before_execute(&s.reg, &instruction_info);
    let cycles = instruction.execute(s);
    monitor.on_after_execute(&s.reg, &instruction_info, cycles);
    cycles
}

#[cfg(test)]
mod tests {
    use crate::{
        get, p, set, step, Cpu, DummyMonitor, Memory, Opcode, Reg, VmState, IRQ, OSWRCH, P,
    };

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

    #[test]
    fn brk() {
        let mut s = VmState::default();
        s.reg.pc = 0x1000;
        s.memory[0x1000] = Opcode::Brk as u8;
        s.memory.store_word(IRQ, 0x9876);
        set!(s.reg, B, false);
        let cycles = step(&DummyMonitor, &Cpu::make_6502(), &mut s);
        assert_eq!(7, cycles);
        assert!(get!(s.reg, B));
        assert_eq!(0x9876, s.reg.pc);
    }

    #[test]
    fn jsr_software_interrupt() {
        const START: u16 = 0x1000;
        const OS: u16 = 0x2000;
        let p_test = P::D | P::ONE;

        let monitor = DummyMonitor;
        let cpu = Cpu::make_6502();
        let mut s = VmState::default();

        s.memory.store_word(IRQ, OS);

        // Set up OSWRCH as a software interrupt
        s.memory[OSWRCH] = Opcode::Brk as u8;
        s.memory[OSWRCH + 1] = Opcode::Nop as u8;
        s.memory[OSWRCH + 2] = Opcode::Rts as u8;

        s.memory[START] = Opcode::Jsr as u8;
        s.memory.store_word(START + 1, OSWRCH);

        s.reg.pc = START;
        s.reg.p = p_test;
        set!(s.reg, B, false);

        let cycles = step(&monitor, &cpu, &mut s);
        assert_eq!(6, cycles);
        assert!(!get!(s.reg, B));
        assert_eq!(OSWRCH, s.reg.pc);

        let cycles = step(&monitor, &cpu, &mut s);
        assert_eq!(7, cycles);
        assert!(get!(s.reg, B));
        assert_eq!(OS, s.reg.pc);

        assert_eq!(p_test.bits(), s.peek());
        assert_eq!(OSWRCH + 1, s.peek_back_word(1));
    }
}
