use crate::{get, Cpu, DummyMonitor, Instruction, InstructionInfo, Monitor, VmState};

pub(crate) struct Vm {
    pub(crate) monitor: Box<dyn Monitor>,
    pub(crate) cpu: Cpu,
    pub(crate) s: VmState,
    pub(crate) cycles: u32,
}

impl Default for Vm {
    fn default() -> Self {
        Self::new(Box::new(DummyMonitor), Cpu::make_6502(), VmState::default())
    }
}

impl Vm {
    #[allow(unused)]
    pub(crate) fn new(monitor: Box<dyn Monitor>, cpu: Cpu, s: VmState) -> Self {
        Self {
            monitor,
            cpu,
            s,
            cycles: 0,
        }
    }

    #[allow(unused)]
    pub(crate) fn with_vm_state(s: VmState) -> Self {
        Self::new(Box::new(DummyMonitor), Cpu::make_6502(), s)
    }

    #[allow(unused)]
    #[must_use]
    pub(crate) fn step(&mut self) -> bool {
        self.monitor.on_before_fetch(&self.s.reg);
        let instruction = Instruction::fetch(&self.cpu, &mut self.s);
        let instruction_info = InstructionInfo::from_instruction(&instruction);
        self.monitor
            .on_before_execute(&self.cpu, &self.s.reg, &instruction_info);
        let cycles = instruction.execute(&mut self.s);
        self.monitor
            .on_after_execute(&self.cpu, &self.s.reg, &instruction_info, cycles);
        self.cycles += cycles as u32;
        !get!(self.s.reg, B)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        get, p, set, set_up_os, Cpu, DummyMonitor, Image, Memory, Monitor, Opcode, Reg,
        TracingMonitor, Vm, VmState, IRQ, OSWRCH, P,
    };
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn no_operand() {
        let mut vm = Vm::with_vm_state(VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        });
        vm.s.memory[0x0000] = Opcode::Nop as u8;
        assert!(vm.step());
        assert_eq!(2, vm.cycles);
        assert_eq!(0x12, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0001, vm.s.reg.pc)
    }

    #[test]
    fn byte0() {
        let mut vm = Vm::with_vm_state(VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        });
        vm.s.memory[0x0000] = Opcode::AdcImm as u8;
        vm.s.memory[0x0001] = 0x34;
        assert!(vm.step());
        assert_eq!(2, vm.cycles);
        assert_eq!(0x46, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0002, vm.s.reg.pc)
    }

    #[test]
    fn byte1() {
        let mut vm = Vm::with_vm_state(VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        });
        vm.s.memory[0x0000] = Opcode::AdcZp as u8;
        vm.s.memory[0x0001] = 0x34;
        vm.s.memory[0x0034] = 0x56;
        assert!(vm.step());
        assert_eq!(3, vm.cycles);
        assert_eq!(0x68, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0002, vm.s.reg.pc)
    }

    #[test]
    fn word0() {
        let mut vm = Vm::with_vm_state(VmState {
            reg: Reg {
                a: 0x12,
                ..Default::default()
            },
            memory: Memory::new(),
        });
        vm.s.memory[0x0000] = Opcode::JmpAbs as u8;
        vm.s.memory[0x0001] = 0x00;
        vm.s.memory[0x0002] = 0x10;
        assert!(vm.step());
        assert_eq!(3, vm.cycles);
        assert_eq!(0x12, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x1000, vm.s.reg.pc)
    }

    #[test]
    fn word1() {
        let mut vm = Vm::with_vm_state(VmState {
            reg: Reg {
                a: 0x25,
                ..Default::default()
            },
            memory: Memory::new(),
        });
        vm.s.memory[0x0000] = Opcode::AdcAbs as u8;
        vm.s.memory[0x0001] = 0x12;
        vm.s.memory[0x0002] = 0x34;
        vm.s.memory[0x3412] = 0x13;
        assert!(vm.step());
        assert_eq!(4, vm.cycles);
        assert_eq!(0x38, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0003, vm.s.reg.pc)
    }

    #[test]
    fn brk() {
        let mut vm = Vm::default();
        vm.s.reg.pc = 0x1000;
        vm.s.memory[0x1000] = Opcode::Brk as u8;
        vm.s.memory.store_word(IRQ, 0x9876);
        set!(vm.s.reg, B, false);
        assert!(!vm.step());
        assert_eq!(7, vm.cycles);
        assert!(get!(vm.s.reg, B));
        assert_eq!(0x9876, vm.s.reg.pc);
    }

    #[test]
    fn jsr_software_interrupt() {
        const START: u16 = 0x1000;
        const OS: u16 = 0x2000;
        let p_test = P::D | P::ONE;

        let mut vm = Vm::default();
        set_up_os(&mut vm, OS);

        vm.s.memory[START] = Opcode::Jsr as u8;
        vm.s.memory.store_word(START + 1, OSWRCH);

        vm.s.reg.pc = START;
        vm.s.reg.p = p_test;
        set!(vm.s.reg, B, false);

        assert!(vm.step());
        assert_eq!(6, vm.cycles);
        assert!(!get!(vm.s.reg, B));
        assert_eq!(OSWRCH, vm.s.reg.pc);

        assert!(!vm.step());
        assert_eq!(13, vm.cycles);
        assert_eq!(Some(OSWRCH), os_brk_addr(&vm, OS));
    }

    #[rstest]
    #[case(false, "HELLO, WORLD!", include_str!("../../examples/hello-world.r6502.txt"))]
    #[case(false, "Hello, world\r\n", include_str!("../../examples/test.r6502.txt"))]
    #[case(false, "Hello, world\r\n", include_str!("../../examples/test-optimized.r6502.txt"))]
    #[case(false, "String0\nString1\n", include_str!("../../examples/strings.r6502.txt"))]
    fn stdout(
        #[case] trace: bool,
        #[case] expected_stdout: &str,
        #[case] input: &str,
    ) -> Result<()> {
        assert_eq!(expected_stdout, capture_stdout(input, trace)?);
        Ok(())
    }

    fn os_brk_addr(vm: &Vm, os: u16) -> Option<u16> {
        if get!(vm.s.reg, B) && vm.s.reg.pc == os {
            let addr = vm.s.peek_back_word(1).wrapping_sub(1);
            Some(addr)
        } else {
            None
        }
    }

    fn capture_stdout(input: &str, trace: bool) -> Result<String> {
        const OS: u16 = 0x2000;
        const RETURN_ADDR: u16 = 0x1234;

        let image = input.parse::<Image>()?;

        let monitor: Box<dyn Monitor> = if trace {
            Box::new(TracingMonitor)
        } else {
            Box::new(DummyMonitor)
        };

        let mut vm = Vm::new(monitor, Cpu::make_6502(), VmState::default());
        let rts = vm
            .cpu
            .get_op_info(&Opcode::Rts)
            .expect("RTS must exist")
            .clone();
        set_up_os(&mut vm, OS);
        vm.s.memory.load(&image);
        vm.s.push_word(RETURN_ADDR - 1);
        vm.s.reg.pc = image.origin;
        set!(vm.s.reg, B, false);

        let mut result = String::new();
        loop {
            while vm.step() {}

            match os_brk_addr(&vm, OS) {
                Some(RETURN_ADDR) => break,
                Some(OSWRCH) => {
                    result.push(vm.s.reg.a as char);
                    if trace {
                        println!("stdout={result}");
                    }

                    // Is this equivalent to RTI?
                    vm.s.pull();
                    vm.s.pull_word();
                    set!(vm.s.reg, B, false);
                    rts.op.execute_no_operand(&mut vm.s);
                }
                _ => panic!("expectation failed"),
            }
        }

        Ok(result)
    }
}
