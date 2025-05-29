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
    #[case(
        false,
        "HELLO, WORLD!",
        r#" 0E00  A2 00     LDX  #$00
 0E02  BD 0E 0E  LDA  $0E0E, X
 0E05  F0 06     BEQ  $0E0D
 0E07  20 EE FF  JSR  $FFEE
 0E0A  E8        INX
 0E0B  D0 F5     BNE  $0E02
 0E0D  60        RTS
 0E0E  48 45 4C 4C 4F 2C 20 57 4F 52 4C 44 21 00        |HELLO, WORLD!.  |
"#
    )]
    #[case(
        false,
        "Hello, world\r\n",
        r#" 2000  A2 00     LDX  #$00
 2002  BD 11 20  LDA  $2011, X
 2005  C9 00     CMP  #$00
 2007  F0 07     BEQ  $2010
 2009  20 EE FF  JSR  $FFEE
 200C  E8        INX
 200D  4C 02 20  JMP  $2002
 2010  60        RTS
 2011  48 65 6C 6C 6F 2C 20 77 6F 72 6C 64 0D 0A 00     |Hello, world... |
"#
    )]
    #[case(
        false,
        "String0\nString1\n",
        r#" 0E00  A9 4D     LDA  #$4D
 0E02  85 80     STA  $80
 0E04  A9 0E     LDA  #$0E
 0E06  85 81     STA  $81
 0E08  20 0C 0E  JSR  $0E0C
 0E0B  60        RTS  
 0E0C  A0 00     LDY  #$00
 0E0E  B1 80     LDA  ($80), Y
 0E10  AA        TAX  
 0E11  E0 00     CPX  #$00
 0E13  F0 15     BEQ  $0E2A
 0E15  C8        INY  
 0E16  B1 80     LDA  ($80), Y
 0E18  85 82     STA  $82
 0E1A  C8        INY  
 0E1B  B1 80     LDA  ($80), Y
 0E1D  85 83     STA  $83
 0E1F  98        TYA  
 0E20  48        PHA  
 0E21  20 2B 0E  JSR  $0E2B
 0E24  68        PLA  
 0E25  A8        TAY  
 0E26  CA        DEX  
 0E27  4C 11 0E  JMP  $0E11
 0E2A  60        RTS  
 0E2B  A0 00     LDY  #$00
 0E2D  B1 82     LDA  ($82), Y
 0E2F  C9 00     CMP  #$00
 0E31  F0 07     BEQ  $0E3A
 0E33  20 EE FF  JSR  $FFEE
 0E36  C8        INY  
 0E37  4C 2D 0E  JMP  $0E2D
 0E3A  60        RTS  
 0E3B  53 74 72 69 6E 67 30 0A 00 53 74 72 69 6E 67 31  |String0..String1|
 0E4B  0A 00 02 3B 0E 44 0E                             |...;.D.         |
"#
    )]
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
