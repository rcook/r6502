use crate::{p_get, DummyMonitor, Instruction, InstructionInfo, Monitor, TotalCycles, VmState};
use derive_builder::Builder;
use std::result::Result as StdResult;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Vm {
    #[builder(default = "self.default_monitor()?")]
    pub monitor: Box<dyn Monitor>,

    #[builder(default)]
    pub s: VmState,

    #[builder(default = "0")]
    pub total_cycles: TotalCycles,
}

impl Default for Vm {
    fn default() -> Self {
        VmBuilder::default().build().expect("Must succeed")
    }
}

impl Vm {
    /*
    pub(crate) fn new(monitor: Box<dyn Monitor>, s: VmState, total_cycles: TotalCycles) -> Self {
        Self {
            monitor,
            s,
            total_cycles,
        }
    }
    */

    #[must_use]
    pub fn step(&mut self) -> bool {
        let instruction = Instruction::fetch(&self.s);
        let instruction_info = InstructionInfo::from_instruction(&instruction);
        self.monitor.on_before_execute(
            self.total_cycles,
            self.s.reg.clone(),
            instruction_info.clone(),
        );
        let instruction_cycles = instruction.execute(&mut self.s);
        self.monitor.on_after_execute(
            self.total_cycles,
            self.s.reg.clone(),
            instruction_info.clone(),
        );
        self.total_cycles += instruction_cycles as TotalCycles;
        !p_get!(self.s.reg, I)
    }
}

impl VmBuilder {
    fn default_monitor(&self) -> StdResult<Box<dyn Monitor>, VmBuilderError> {
        Ok(Box::new(DummyMonitor))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        p, p_get, p_set, DummyMonitor, Image, Monitor, Opcode, OsBuilder, TracingMonitor, Vm,
        VmBuilder, IRQ, MOS_6502, OSWRCH, P,
    };
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn no_operand() {
        let mut vm = Vm::default();
        vm.s.reg.a = 0x12;
        vm.s.memory[0x0000] = Opcode::Nop as u8;
        assert!(vm.step());
        assert_eq!(2, vm.total_cycles);
        assert_eq!(0x12, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0001, vm.s.reg.pc)
    }

    #[test]
    fn byte0() {
        let mut vm = Vm::default();
        vm.s.reg.a = 0x12;
        vm.s.memory[0x0000] = Opcode::AdcImm as u8;
        vm.s.memory[0x0001] = 0x34;
        assert!(vm.step());
        assert_eq!(2, vm.total_cycles);
        assert_eq!(0x46, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0002, vm.s.reg.pc)
    }

    #[test]
    fn byte1() {
        let mut vm = Vm::default();
        vm.s.reg.a = 0x12;
        vm.s.memory[0x0000] = Opcode::AdcZp as u8;
        vm.s.memory[0x0001] = 0x34;
        vm.s.memory[0x0034] = 0x56;
        assert!(vm.step());
        assert_eq!(3, vm.total_cycles);
        assert_eq!(0x68, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x0002, vm.s.reg.pc)
    }

    #[test]
    fn word0() {
        let mut vm = Vm::default();
        vm.s.reg.a = 0x12;
        vm.s.memory[0x0000] = Opcode::JmpAbs as u8;
        vm.s.memory[0x0001] = 0x00;
        vm.s.memory[0x0002] = 0x10;
        assert!(vm.step());
        assert_eq!(3, vm.total_cycles);
        assert_eq!(0x12, vm.s.reg.a);
        assert_eq!(p!(), vm.s.reg.p);
        assert_eq!(0x1000, vm.s.reg.pc)
    }

    #[test]
    fn word1() {
        let mut vm = Vm::default();
        vm.s.reg.a = 0x25;
        vm.s.memory[0x0000] = Opcode::AdcAbs as u8;
        vm.s.memory[0x0001] = 0x12;
        vm.s.memory[0x0002] = 0x34;
        vm.s.memory[0x3412] = 0x13;
        assert!(vm.step());
        assert_eq!(4, vm.total_cycles);
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
        p_set!(vm.s.reg, B, false);
        assert!(!vm.step());
        assert_eq!(7, vm.total_cycles);
        assert!(!p_get!(vm.s.reg, B));
        assert_eq!(0x9876, vm.s.reg.pc);
    }

    #[test]
    fn jsr_brk() -> Result<()> {
        const OS_ADDR: u16 = 0x7000;
        const START: u16 = 0x1000;
        let p_test = P::D | P::ALWAYS_ONE;

        let mut vm = Vm::default();
        let os = OsBuilder::default().os_addr(OS_ADDR).build()?;
        os.load_into_vm(&mut vm);

        vm.s.memory[START] = Opcode::Jsr as u8;
        vm.s.memory.store_word(START + 1, OSWRCH);

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
        assert_eq!(OS_ADDR, vm.s.reg.pc);
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
        let mut vm = load_into_vm(include_str!("../../examples/add8.r6502.txt"))?;
        vm.s.reg.pc = 0x0e01;
        while vm.step() {}
        assert_eq!(21, vm.total_cycles);
        assert_eq!(0x46, vm.s.memory[0x0e00]);
        Ok(())
    }

    #[test]
    fn add16() -> Result<()> {
        let mut vm = load_into_vm(include_str!("../../examples/add16.r6502.txt"))?;
        vm.s.reg.pc = 0x0e02;
        while vm.step() {}
        assert_eq!(33, vm.total_cycles);
        assert_eq!(0xac68, vm.s.memory.fetch_word(0x0e00));
        Ok(())
    }

    #[test]
    fn div16() -> Result<()> {
        const NUM1: u16 = 0x0e33;
        const REM: u16 = 0x0e37;

        let mut vm = load_into_vm(include_str!("../../examples/div16.r6502.txt"))?;
        vm.s.reg.pc = 0x0e02;
        while vm.step() {}
        assert_eq!(893, vm.total_cycles);
        let quotient = vm.s.memory.fetch_word(NUM1);
        let remainder = vm.s.memory.fetch_word(REM);
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

        let mut vm = VmBuilder::default().monitor(monitor).build()?;
        let image = input.parse::<Image>()?;
        vm.s.memory.load(&image);
        vm.s.reg.pc = image.start;

        let os = OsBuilder::default()
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

    fn load_into_vm(input: &str) -> Result<Vm> {
        let image = input.parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        let mut vm = Vm::default();
        vm.s.memory.load(&image);
        Ok(vm)
    }
}
