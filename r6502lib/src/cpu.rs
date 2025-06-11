use crate::util::{make_word, split_word};
use crate::{
    p_get, BusView, DummyMonitor, Frequency, Instruction, InstructionInfo, Monitor, Reg,
    TotalCycles, R6502_DUMP_MAGIC_NUMBERS, STACK_BASE,
};
use anyhow::Result;
use log::{debug, log_enabled, Level};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

const CPU_FREQUENCY: Frequency = Frequency::MHz(1_000_000);
static CPU_TICK: LazyLock<Duration> = LazyLock::new(|| CPU_FREQUENCY.get_tick());

pub struct Cpu<'a> {
    pub reg: Reg,
    pub bus: BusView<'a>,
    pub total_cycles: TotalCycles,
    monitor: Box<dyn Monitor>,
}

impl<'a> Cpu<'a> {
    #[must_use]
    pub fn new(bus: BusView<'a>, monitor: Option<Box<dyn Monitor>>) -> Self {
        Self {
            reg: Reg::default(),
            bus,
            total_cycles: 0,
            monitor: monitor.unwrap_or_else(|| Box::new(DummyMonitor)),
        }
    }

    pub fn write_snapshot(&self, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        let (hi, lo) = split_word(self.reg.pc);
        writer.write_all(&R6502_DUMP_MAGIC_NUMBERS)?;
        writer.write_all(&[
            self.bus.machine_type() as u8,
            lo,
            hi,
            self.reg.a,
            self.reg.x,
            self.reg.y,
            self.reg.sp,
            self.reg.p.bits(),
        ])?;

        // There must be a more efficient way of doing this...
        for addr in 0..=0xffff {
            writer.write_all(&[self.bus.load(addr)])?;
        }

        Ok(())
    }

    #[must_use]
    pub fn step(&mut self) -> bool {
        let instruction = Instruction::fetch(self);
        let instruction_info = InstructionInfo::from_instruction(&instruction);
        self.monitor.on_before_execute(
            self.total_cycles,
            self.reg.clone(),
            instruction_info.clone(),
        );

        if log_enabled!(Level::Debug) {
            debug!("{:?}", instruction_info);
        }

        let before = Instant::now();
        let instruction_cycles = instruction.execute(self);

        let d = *CPU_TICK * instruction_cycles as u32;

        // Is there a better way to do this?
        loop {
            let now = Instant::now();
            let elapsed = now - before;
            if elapsed >= d {
                break;
            }
        }

        self.monitor.on_after_execute(
            self.total_cycles,
            self.reg.clone(),
            instruction_info.clone(),
        );
        self.total_cycles += instruction_cycles as TotalCycles;
        !p_get!(self.reg, I)
    }

    pub fn push(&mut self, value: u8) {
        self.set_stack_value(value);
        self.reg.sp = self.reg.sp.wrapping_sub(1);
    }

    #[must_use]
    pub fn pull(&mut self) -> u8 {
        self.reg.sp = self.reg.sp.wrapping_add(1);
        self.get_stack_value()
    }

    pub fn push_word(&mut self, value: u16) {
        let (hi, lo) = split_word(value);
        self.push(hi);
        self.push(lo);
    }

    #[must_use]
    pub fn pull_word(&mut self) -> u16 {
        let lo = self.pull();
        let hi = self.pull();
        make_word(hi, lo)
    }

    #[cfg(test)]
    #[must_use]
    pub(crate) fn peek_word(&self) -> u16 {
        self.peek_back_word(0x00)
    }

    #[must_use]
    pub(crate) fn peek_back_word(&self, offset: u8) -> u16 {
        let stack_addr = (STACK_BASE + self.reg.sp as u16).wrapping_add(offset as u16);
        let hi = self.bus.load(stack_addr.wrapping_add(2));
        let lo = self.bus.load(stack_addr.wrapping_add(1));
        make_word(hi, lo)
    }

    #[must_use]
    fn get_stack_value(&self) -> u8 {
        self.bus.load(STACK_BASE.wrapping_add(self.reg.sp as u16))
    }

    fn set_stack_value(&mut self, value: u8) {
        self.bus
            .store(STACK_BASE.wrapping_add(self.reg.sp as u16), value);
    }
}

#[cfg(test)]
mod tests {
    use crate::util::make_word;
    use crate::{
        p, p_get, p_set, Bus, Cpu, Image, MachineType, Monitor, Opcode, Os, TracingMonitor, IRQ,
        MOS_6502, OSWRCH, P,
    };
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn no_operand() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::Nop as u8);
        assert!(cpu.step());
        assert_eq!(2, cpu.total_cycles);
        assert_eq!(0x12, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0001, cpu.reg.pc);
    }

    #[test]
    fn byte0() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::AdcImm as u8);
        bus.store(0x0001, 0x34);
        assert!(cpu.step());
        assert_eq!(2, cpu.total_cycles);
        assert_eq!(0x46, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0002, cpu.reg.pc);
    }

    #[test]
    fn byte1() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::AdcZp as u8);
        bus.store(0x0001, 0x34);
        bus.store(0x0034, 0x56);
        assert!(cpu.step());
        assert_eq!(3, cpu.total_cycles);
        assert_eq!(0x68, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0002, cpu.reg.pc);
    }

    #[test]
    fn word0() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::JmpAbs as u8);
        bus.store(0x0001, 0x00);
        bus.store(0x0002, 0x10);
        assert!(cpu.step());
        assert_eq!(3, cpu.total_cycles);
        assert_eq!(0x12, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x1000, cpu.reg.pc);
    }

    #[test]
    fn word1() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.a = 0x25;
        bus.store(0x0000, Opcode::AdcAbs as u8);
        bus.store(0x0001, 0x12);
        bus.store(0x0002, 0x34);
        bus.store(0x3412, 0x13);
        assert!(cpu.step());
        assert_eq!(4, cpu.total_cycles);
        assert_eq!(0x38, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0003, cpu.reg.pc);
    }

    #[test]
    fn brk() {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        cpu.reg.pc = 0x1000;
        bus.store(0x1000, Opcode::Brk as u8);
        bus.store(IRQ, 0x76);
        bus.store(IRQ + 1, 0x98);
        p_set!(cpu.reg, B, false);
        assert!(!cpu.step());
        assert_eq!(7, cpu.total_cycles);
        assert!(!p_get!(cpu.reg, B));
        assert_eq!(0x9876, cpu.reg.pc);
    }

    #[test]
    fn jsr_brk() {
        const START: u16 = 0x1000;
        let p_test = P::D | P::ALWAYS_ONE;

        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);

        let os = Os::new(MachineType::Acorn);
        os.load_into_vm(&mut cpu);

        let irq_addr = os.irq_addr.expect("Must have value");

        bus.store(START, Opcode::Jsr as u8);
        bus.store(START + 1, OSWRCH as u8);
        bus.store(START + 2, (OSWRCH >> 8) as u8);

        cpu.reg.pc = START;
        cpu.reg.p = p_test;
        p_set!(cpu.reg, B, false);

        assert!(cpu.step());
        assert_eq!(6, cpu.total_cycles);
        assert!(!p_get!(cpu.reg, B));
        assert_eq!(OSWRCH, cpu.reg.pc);

        assert!(!cpu.step());
        assert_eq!(13, cpu.total_cycles);
        assert!(!p_get!(cpu.reg, B));
        assert_eq!(irq_addr, cpu.reg.pc);
        assert_eq!(Some(OSWRCH), os.is_os_vector(&cpu));
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
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        let image = include_str!("../../examples/add8.r6502.txt").parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        bus.store_image(&image)?;
        cpu.reg.pc = 0x0e01;
        while cpu.step() {}
        assert_eq!(21, cpu.total_cycles);
        assert_eq!(0x46, bus.load(0x0e00));
        Ok(())
    }

    #[test]
    fn add16() -> Result<()> {
        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        let image = include_str!("../../examples/add16.r6502.txt").parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        bus.store_image(&image)?;
        cpu.reg.pc = 0x0e02;
        while cpu.step() {}
        assert_eq!(33, cpu.total_cycles);
        let lo = bus.load(0x0e00);
        let hi = bus.load(0x0e01);
        assert_eq!(0xac68, make_word(hi, lo));
        Ok(())
    }

    #[test]
    fn div16() -> Result<()> {
        const NUM1: u16 = 0x0e33;
        const REM: u16 = 0x0e37;

        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), None);
        let image = include_str!("../../examples/div16.r6502.txt").parse::<Image>()?;
        assert_eq!(0x0e00, image.load);
        bus.store_image(&image)?;
        cpu.reg.pc = 0x0e02;
        while cpu.step() {}
        assert_eq!(893, cpu.total_cycles);
        let lo = bus.load(NUM1);
        let hi = bus.load(NUM1 + 1);
        let quotient = make_word(hi, lo);
        let lo = bus.load(REM);
        let hi = bus.load(REM + 1);
        let remainder = make_word(hi, lo);
        assert_eq!(0x01d2, quotient);
        assert_eq!(0x0000, remainder);
        Ok(())
    }

    fn capture_stdout(input: &str, trace: bool) -> Result<String> {
        let monitor: Option<Box<dyn Monitor>> = if trace {
            Some(Box::new(TracingMonitor::default()))
        } else {
            None
        };

        let bus = Bus::default();
        let mut cpu = Cpu::new(bus.view(), monitor);
        let image = input.parse::<Image>()?;
        bus.store_image(&image)?;
        cpu.reg.pc = image.start;

        let os = Os::new(MachineType::Acorn);
        os.load_into_vm(&mut cpu);

        let return_addr = os.return_addr.expect("Must have value");

        let rts = MOS_6502
            .get_op_info(&Opcode::Rts)
            .expect("RTS must exist")
            .clone();

        cpu.push_word(return_addr - 1);
        p_set!(cpu.reg, B, false);

        let mut result = String::new();
        loop {
            while cpu.step() {}

            match os.is_os_vector(&cpu) {
                Some(addr) if addr == return_addr => break,
                Some(OSWRCH) => {
                    result.push(cpu.reg.a as char);
                    if trace {
                        println!("stdout={result}");
                    }

                    // Is this equivalent to RTI?
                    _ = cpu.pull();
                    _ = cpu.pull_word();
                    p_set!(cpu.reg, I, false);
                    rts.execute_no_operand(&mut cpu);
                }
                _ => panic!("expectation failed"),
            }
        }

        Ok(result)
    }
}
