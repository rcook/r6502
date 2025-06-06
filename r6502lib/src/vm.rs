use crate::{p_get, Instruction, InstructionInfo, Monitor, TotalCycles, VmState};

pub struct Vm<'a> {
    pub monitor: Box<dyn Monitor>,
    pub s: VmState<'a>,
    pub total_cycles: TotalCycles,
}

impl<'a> Vm<'a> {
    pub fn new(monitor: Box<dyn Monitor>, s: VmState<'a>) -> Self {
        Self {
            monitor,
            s,
            total_cycles: 0,
        }
    }

    #[must_use]
    pub fn step(&mut self) -> bool {
        let instruction = Instruction::fetch(&self.s);
        let instruction_info = InstructionInfo::from_instruction(&instruction);
        self.monitor.on_before_execute(
            self.total_cycles,
            self.s.reg.clone(),
            instruction_info.clone(),
        );
        //let before = Instant::now();
        let instruction_cycles = instruction.execute(&mut self.s);
        //let after = Instant::now();
        //let d0 = after - before;
        //let d1 = Duration::from_micros(instruction_cycles as u64).saturating_sub(d0);
        //sleep(d1);
        self.monitor.on_after_execute(
            self.total_cycles,
            self.s.reg.clone(),
            instruction_info.clone(),
        );
        self.total_cycles += instruction_cycles as TotalCycles;
        !p_get!(self.s.reg, I)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::make_word;
    use crate::{
        p, p_get, p_set, DummyMonitor, Image, Memory, Monitor, Opcode, OsBuilder, Reg,
        TracingMonitor, Vm, VmState, IRQ, MOS_6502, OSWRCH, P,
    };
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn no_operand() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        vm.s.reg.a = 0x12;
        memory.store(0x0000, Opcode::Nop as u8);
        assert!(vm.step());
        assert_eq!(2, vm.total_cycles);
        assert_eq!(0x12, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0001, vm.s.reg.pc);
        Ok(())
    }

    #[test]
    fn byte0() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        vm.s.reg.a = 0x12;
        memory.store(0x0000, Opcode::AdcImm as u8);
        memory.store(0x0001, 0x34);
        assert!(vm.step());
        assert_eq!(2, vm.total_cycles);
        assert_eq!(0x46, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0002, vm.s.reg.pc);
        Ok(())
    }

    #[test]
    fn byte1() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        vm.s.reg.a = 0x12;
        memory.store(0x0000, Opcode::AdcZp as u8);
        memory.store(0x0001, 0x34);
        memory.store(0x0034, 0x56);
        assert!(vm.step());
        assert_eq!(3, vm.total_cycles);
        assert_eq!(0x68, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0002, vm.s.reg.pc);
        Ok(())
    }

    #[test]
    fn word0() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        vm.s.reg.a = 0x12;
        memory.store(0x0000, Opcode::JmpAbs as u8);
        memory.store(0x0001, 0x00);
        memory.store(0x0002, 0x10);
        assert!(vm.step());
        assert_eq!(3, vm.total_cycles);
        assert_eq!(0x12, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x1000, vm.s.reg.pc);
        Ok(())
    }

    #[test]
    fn word1() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        vm.s.reg.a = 0x25;
        memory.store(0x0000, Opcode::AdcAbs as u8);
        memory.store(0x0001, 0x12);
        memory.store(0x0002, 0x34);
        memory.store(0x3412, 0x13);
        assert!(vm.step());
        assert_eq!(4, vm.total_cycles);
        assert_eq!(0x38, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0003, vm.s.reg.pc);
        Ok(())
    }

    #[test]
    fn brk() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        vm.s.reg.pc = 0x1000;
        memory.store(0x1000, Opcode::Brk as u8);
        memory.store(IRQ, 0x76);
        memory.store(IRQ + 1, 0x98);
        p_set!(vm.s.reg, B, false);
        assert!(!vm.step());
        assert_eq!(7, vm.total_cycles);
        assert!(!p_get!(vm.s.reg, B));
        assert_eq!(0x9876, vm.s.reg.pc);
        Ok(())
    }

    #[test]
    fn jsr_brk() -> Result<()> {
        const IRQ_ADDR: u16 = 0x7000;
        const START: u16 = 0x1000;
        let p_test = P::D | P::ALWAYS_ONE;

        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        let os = OsBuilder::default()
            .irq_addr(IRQ_ADDR)
            .os_vectors(vec![OSWRCH])
            .build()?;
        os.load_into_vm(&mut vm);

        memory.store(START, Opcode::Jsr as u8);
        memory.store(START + 1, OSWRCH as u8);
        memory.store(START + 2, (OSWRCH >> 8) as u8);

        vm.s.reg.pc = START;
        vm.s.reg.p = p_test;
        p_set!(vm.s.reg, B, false);

        assert!(vm.step());
        assert_eq!(6, vm.total_cycles);
        assert!(!p_get!(vm.s.reg, B));
        assert_eq!(OSWRCH, vm.s.reg.pc);

        assert!(!vm.step());
        assert_eq!(13, vm.total_cycles);
        assert!(!p_get!(vm.s.reg, B));
        assert_eq!(IRQ_ADDR, vm.s.reg.pc);
        assert_eq!(Some(OSWRCH), os.is_os_vector(&vm));

        Ok(())
    }

    #[rstest]
    #[case("HELLO, WORLD!", include_str!("../../examples/hello-world.r6502.txt"))]
    #[case("Hello, world\r\n", include_str!("../../examples/test.r6502.txt"))]
    #[case("Hello, world\r\n", include_str!("../../examples/test-optimized.r6502.txt"))]
    #[case("String0\nString1\n", include_str!("../../examples/strings.r6502.txt"))]
    fn stdout(#[case] expected_stdout: &str, #[case] input: &str) -> Result<()> {
        #[cfg(feature = "not-implemented")]
        const TRACE: bool = true;
        #[cfg(not(feature = "not-implemented"))]
        const TRACE: bool = true;
        assert_eq!(expected_stdout, capture_stdout(input, TRACE)?);
        Ok(())
    }

    #[test]
    fn add8() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        let image = include_str!("../../examples/add8.r6502.txt").parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        memory.store_image(&image)?;
        vm.s.reg.pc = 0x0e01;
        while vm.step() {}
        assert_eq!(21, vm.total_cycles);
        assert_eq!(0x46, memory.load(0x0e00));
        Ok(())
    }

    #[test]
    fn add16() -> Result<()> {
        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        let image = include_str!("../../examples/add16.r6502.txt").parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        memory.store_image(&image)?;
        vm.s.reg.pc = 0x0e02;
        while vm.step() {}
        assert_eq!(33, vm.total_cycles);
        let lo = memory.load(0x0e00);
        let hi = memory.load(0x0e01);
        assert_eq!(0xac68, make_word(hi, lo));
        Ok(())
    }

    #[test]
    fn div16() -> Result<()> {
        const NUM1: u16 = 0x0e33;
        const REM: u16 = 0x0e37;

        let memory = Memory::new();
        let mut vm = Vm::new(
            Box::new(DummyMonitor),
            VmState::new(Reg::default(), memory.view()),
        );

        let image = include_str!("../../examples/div16.r6502.txt").parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        memory.store_image(&image)?;
        vm.s.reg.pc = 0x0e02;
        while vm.step() {}
        assert_eq!(893, vm.total_cycles);
        let lo = memory.load(NUM1);
        let hi = memory.load(NUM1 + 1);
        let quotient = make_word(hi, lo);
        let lo = memory.load(REM);
        let hi = memory.load(REM + 1);
        let remainder = make_word(hi, lo);
        assert_eq!(0x01d2, quotient);
        assert_eq!(0x0000, remainder);
        Ok(())
    }

    fn capture_stdout(input: &str, trace: bool) -> Result<String> {
        const RETURN_ADDR: u16 = 0x1234;

        let monitor: Box<dyn Monitor> = if trace {
            Box::new(TracingMonitor::default())
        } else {
            Box::new(DummyMonitor)
        };

        let memory = Memory::new();
        let mut vm = Vm::new(monitor, VmState::new(Reg::default(), memory.view()));
        let image = input.parse::<Image>()?;
        memory.store_image(&image)?;
        vm.s.reg.pc = image.start;

        let os = OsBuilder::default()
            .irq_addr(0x8000)
            .os_vectors(vec![RETURN_ADDR, OSWRCH])
            .build()?;
        os.load_into_vm(&mut vm);

        let rts = MOS_6502
            .get_op_info(&Opcode::Rts)
            .expect("RTS must exist")
            .clone();

        vm.s.push_word(RETURN_ADDR - 1);
        p_set!(vm.s.reg, B, false);

        let mut result = String::new();
        loop {
            while vm.step() {}

            match os.is_os_vector(&vm) {
                Some(RETURN_ADDR) => break,
                Some(OSWRCH) => {
                    result.push(vm.s.reg.a as char);
                    if trace {
                        println!("stdout={result}");
                    }

                    // Is this equivalent to RTI?
                    _ = vm.s.pull();
                    _ = vm.s.pull_word();
                    p_set!(vm.s.reg, I, false);
                    rts.execute_no_operand(&mut vm.s);
                }
                _ => panic!("expectation failed"),
            }
        }

        Ok(result)
    }
}
