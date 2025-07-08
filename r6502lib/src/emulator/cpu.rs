use crate::emulator::util::{make_word, split_word};
use crate::emulator::{
    BusView, DummyMonitor, Frequency, Instruction, InstructionInfo, IrqEvent, Monitor, Reg,
    TotalCycles, IRQ, STACK_BASE,
};
use log::{debug, log_enabled, Level};
use signal_hook::consts::SIGINT;
use signal_hook::flag::register;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};

const CPU_FREQUENCY: Frequency = Frequency::MHz(3_000_000);
static CPU_TICK: LazyLock<Duration> = LazyLock::new(|| CPU_FREQUENCY.get_tick());

pub struct Cpu<'a> {
    pub reg: Reg,
    pub bus: BusView<'a>,
    pub total_cycles: TotalCycles,
    monitor: Box<dyn Monitor>,
    irq_rx: Receiver<IrqEvent>,
    interrupt: Arc<AtomicBool>,
}

impl<'a> Cpu<'a> {
    #[must_use]
    pub fn new(
        bus: BusView<'a>,
        monitor: Option<Box<dyn Monitor>>,
        irq_rx: Receiver<IrqEvent>,
    ) -> Self {
        let interrupt = Arc::new(AtomicBool::new(false));
        register(SIGINT, Arc::clone(&interrupt)).unwrap();
        Self {
            reg: Reg::default(),
            bus,
            total_cycles: 0,
            monitor: monitor.unwrap_or_else(|| Box::new(DummyMonitor)),
            irq_rx,
            interrupt,
        }
    }

    pub fn step_with_monitor_callbacks(&mut self) {
        let (instruction, instruction_info) = self.decode_next();

        self.monitor.on_before_execute(
            self.total_cycles,
            self.reg.clone(),
            instruction_info.clone(),
        );

        let instruction_cycles = instruction.execute(self);
        Self::spin(instruction_cycles);

        self.monitor.on_after_execute(
            self.total_cycles,
            self.reg.clone(),
            instruction_info.clone(),
        );

        self.total_cycles += TotalCycles::from(instruction_cycles);
    }

    pub fn step(&mut self) {
        let (instruction, _) = self.decode_next();
        let instruction_cycles = instruction.execute(self);
        Self::spin(instruction_cycles);
        self.total_cycles += TotalCycles::from(instruction_cycles);
    }

    pub fn step_no_spin(&mut self) {
        let (instruction, _) = self.decode_next();
        let instruction_cycles = instruction.execute(self);
        self.total_cycles += TotalCycles::from(instruction_cycles);
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
    pub fn peek_word(&self) -> u16 {
        self.peek_back_word(0x00)
    }

    #[must_use]
    pub fn peek_back_word(&self, offset: u8) -> u16 {
        let stack_addr = (STACK_BASE + u16::from(self.reg.sp)).wrapping_add(u16::from(offset));
        let hi = self.bus.load(stack_addr.wrapping_add(2));
        let lo = self.bus.load(stack_addr.wrapping_add(1));
        make_word(hi, lo)
    }

    fn spin(instruction_cycles: u8) {
        let before = Instant::now();
        let d = *CPU_TICK * u32::from(instruction_cycles);

        // Is there a better way to do this?
        loop {
            let now = Instant::now();
            let elapsed = now - before;
            if elapsed >= d {
                break;
            }
        }
    }

    #[must_use]
    fn get_stack_value(&self) -> u8 {
        self.bus
            .load(STACK_BASE.wrapping_add(u16::from(self.reg.sp)))
    }

    fn set_stack_value(&mut self, value: u8) {
        self.bus
            .store(STACK_BASE.wrapping_add(u16::from(self.reg.sp)), value);
    }

    fn decode_next(&mut self) -> (Instruction, InstructionInfo) {
        match self.irq_rx.try_recv() {
            Ok(IrqEvent::Irq) => {
                // IRQ signalled: what to do now?
                self.handle_interrupt();
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                // TBD: IRQ channel will never be connected when using
                // Pia instead of Via. Handle that more gracefully.
            }
        }

        if self.interrupt.load(Ordering::Relaxed) {
            self.interrupt.store(false, Ordering::Relaxed);
            self.handle_interrupt();
        }

        let instruction = Instruction::fetch(self);
        let instruction_info = InstructionInfo::from_instruction(&instruction);
        if log_enabled!(Level::Debug) {
            debug!("{instruction_info:?}");
        }

        (instruction, instruction_info)
    }

    fn handle_interrupt(&mut self) {
        // Reference: https://www.pagetable.com/?p=410
        self.push_word(self.reg.pc);
        let p = self.reg.p.bits();
        self.push(p & 0b1110_1111);
        let pc_lo = self.bus.load(IRQ);
        let pc_hi = self.bus.load(IRQ.wrapping_add(1));
        self.reg.pc = make_word(pc_hi, pc_lo);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::util::{get_brk_addr, make_word, split_word};
    use crate::emulator::{
        Bus, Cpu, Image, IrqChannel, Monitor, Opcode, TracingMonitor, IRQ, MOS_6502, P,
    };
    use crate::{p, p_get, p_set};
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    fn no_operand() {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::Nop as u8);
        cpu.step_no_spin();
        assert!(!p_get!(cpu.reg, I));
        assert_eq!(2, cpu.total_cycles);
        assert_eq!(0x12, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0001, cpu.reg.pc);
    }

    #[test]
    fn byte0() {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::AdcImm as u8);
        bus.store(0x0001, 0x34);
        cpu.step_no_spin();
        assert!(!p_get!(cpu.reg, I));
        assert_eq!(2, cpu.total_cycles);
        assert_eq!(0x46, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0002, cpu.reg.pc);
    }

    #[test]
    fn byte1() {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::AdcZp as u8);
        bus.store(0x0001, 0x34);
        bus.store(0x0034, 0x56);
        cpu.step_no_spin();
        assert!(!p_get!(cpu.reg, I));
        assert_eq!(3, cpu.total_cycles);
        assert_eq!(0x68, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0002, cpu.reg.pc);
    }

    #[test]
    fn word0() {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.a = 0x12;
        bus.store(0x0000, Opcode::JmpAbs as u8);
        bus.store(0x0001, 0x00);
        bus.store(0x0002, 0x10);
        cpu.step_no_spin();
        assert!(!p_get!(cpu.reg, I));
        assert_eq!(3, cpu.total_cycles);
        assert_eq!(0x12, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x1000, cpu.reg.pc);
    }

    #[test]
    fn word1() {
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.a = 0x25;
        bus.store(0x0000, Opcode::AdcAbs as u8);
        bus.store(0x0001, 0x12);
        bus.store(0x0002, 0x34);
        bus.store(0x3412, 0x13);
        cpu.step_no_spin();
        assert!(!p_get!(cpu.reg, I));
        assert_eq!(4, cpu.total_cycles);
        assert_eq!(0x38, cpu.reg.a);
        assert_eq!(p!(), cpu.reg.p);
        assert_eq!(0x0003, cpu.reg.pc);
    }

    #[test]
    fn brk() {
        const IRQ_ADDR: u16 = 0x9876;
        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);
        cpu.reg.pc = 0x1000;
        bus.store(0x1000, Opcode::Brk as u8);
        bus.store(IRQ_ADDR, 0xea);
        let (hi, lo) = split_word(IRQ_ADDR);
        bus.store(IRQ, lo);
        bus.store(IRQ.wrapping_add(1), hi);
        p_set!(cpu.reg, B, false);
        cpu.step_no_spin();
        assert!(p_get!(cpu.reg, I));
        assert_eq!(7, cpu.total_cycles);
        assert!(!p_get!(cpu.reg, B));
        assert_eq!(0x9876, cpu.reg.pc);
    }

    #[test]
    fn jsr_brk() {
        const START: u16 = 0x1000;
        const IRQ_ADDR: u16 = 0xdead;
        const JUMP_ADDR: u16 = 0x1234;
        let p_test = P::D | P::ALWAYS_ONE;

        let bus = Bus::default();
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);

        bus.store(IRQ_ADDR, 0xea);
        let (hi, lo) = split_word(IRQ_ADDR);
        bus.store(IRQ, lo);
        bus.store(IRQ.wrapping_add(1), hi);

        bus.store(START, Opcode::Jsr as u8);
        let (hi, lo) = split_word(JUMP_ADDR);
        bus.store(START.wrapping_add(1), lo);
        bus.store(START.wrapping_add(2), hi);

        cpu.reg.pc = START;
        cpu.reg.p = p_test;
        p_set!(cpu.reg, B, false);

        cpu.step_no_spin();
        assert!(!p_get!(cpu.reg, I));
        assert_eq!(6, cpu.total_cycles);
        assert!(!p_get!(cpu.reg, B));
        assert_eq!(JUMP_ADDR, cpu.reg.pc);

        cpu.step_no_spin();
        assert!(p_get!(cpu.reg, I));
        assert_eq!(13, cpu.total_cycles);
        assert!(!p_get!(cpu.reg, B));
        assert_eq!(IRQ_ADDR, cpu.reg.pc);
        assert_eq!(Some(JUMP_ADDR), get_brk_addr(&cpu));
    }

    const TEST_PROGRAM_0: &str = r" 0E00  A2 00     LDX  #$00
 0E02  BD 10 0E  LDA  $0E10, X
 0E05  F0 06     BEQ  $0E0D
 0E07  20 EE FF  JSR  $FFEE
 0E0A  E8        INX  
 0E0B  D0 F5     BNE  $0E02
 0E0D  4C C0 FF  JMP  $FFC0
 0E10  48 45 4C 4C 4F 2C 20 57 4F 52 4C 44 21 00        |HELLO, WORLD!.  |
";
    const TEST_PROGRAM_1: &str = r" 2000  A2 00     LDX  #$00
 2002  BD 11 20  LDA  $2011, X
 2005  C9 00     CMP  #$00
 2007  F0 07     BEQ  $2010
 2009  20 EE FF  JSR  $FFEE
 200C  E8        INX  
 200D  4C 02 20  JMP  $2002
 2010  60        RTS  
 2011  48 65 6C 6C 6F 2C 20 77 6F 72 6C 64 0D 0A 00     |Hello, world... |
";
    const TEST_PROGRAM_2: &str = r" 2000  A2 00     LDX  #$00
 2002  BD 0F 20  LDA  $200F, X
 2005  F0 07     BEQ  $200E
 2007  20 EE FF  JSR  $FFEE
 200A  E8        INX  
 200B  4C 02 20  JMP  $2002
 200E  60        RTS  
 200F  48 65 6C 6C 6F 2C 20 77 6F 72 6C 64 0D 0A 00     |Hello, world... |
";
    const TEST_PROGRAM_3: &str = r" 0E00  A9 4D     LDA  #$4D
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
";

    #[rstest]
    #[case("HELLO, WORLD!", TEST_PROGRAM_0)]
    #[case("Hello, world\r\n", TEST_PROGRAM_1)]
    #[case("Hello, world\r\n", TEST_PROGRAM_2)]
    #[case("String0\nString1\n", TEST_PROGRAM_3)]
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
        let image = r" 0E00  00                                               |.               |
 0E01  18        CLC  
 0E02  AD 0C 0E  LDA  $0E0C
 0E05  6D 0D 0E  ADC  $0E0D
 0E08  8D 00 0E  STA  $0E00
 0E0B  00        BRK  
 0E0C  12 34                                            |.4              |
"
        .parse::<Image>()?;

        let load = image.load().expect("Must be set");
        assert_eq!(0x0e00, load);
        let bus = Bus::default_with_image(&image)?;
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);

        let irq_addr = bus.load_irq_unsafe();
        cpu.bus.store(irq_addr, 0xea);

        cpu.reg.pc = load.wrapping_add(1);
        loop {
            cpu.step_no_spin();
            if p_get!(cpu.reg, I) {
                break;
            }
        }
        assert_eq!(21, cpu.total_cycles);
        assert_eq!(0x46, bus.load(0x0e00));
        Ok(())
    }

    #[test]
    fn add16() -> Result<()> {
        let image = r" 0E00  00 00                                            |..              |
 0E02  18        CLC  
 0E03  AD 16 0E  LDA  $0E16
 0E06  6D 18 0E  ADC  $0E18
 0E09  8D 00 0E  STA  $0E00
 0E0C  AD 17 0E  LDA  $0E17
 0E0F  6D 19 0E  ADC  $0E19
 0E12  8D 01 0E  STA  $0E01
 0E15  00        BRK  
 0E16  12 34 56 78                                      |.4Vx            |"
            .parse::<Image>()?;

        let load = image.load().expect("Must be set");
        assert_eq!(0x0e00, load);
        let bus = Bus::default_with_image(&image)?;
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);

        let irq_addr = bus.load_irq_unsafe();
        cpu.bus.store(irq_addr, 0xea);

        cpu.reg.pc = load.wrapping_add(2);
        loop {
            cpu.step_no_spin();
            if p_get!(cpu.reg, I) {
                break;
            }
        }
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

        let image = r" 0E00  A9 00     LDA  #$00
 0E02  8D 37 0E  STA  $0E37
 0E05  8D 38 0E  STA  $0E38
 0E08  A2 10     LDX  #$10
 0E0A  0E 33 0E  ASL  $0E33
 0E0D  2E 34 0E  ROL  $0E34
 0E10  2E 37 0E  ROL  $0E37
 0E13  2E 38 0E  ROL  $0E38
 0E16  AD 37 0E  LDA  $0E37
 0E19  38        SEC  
 0E1A  ED 35 0E  SBC  $0E35
 0E1D  A8        TAY  
 0E1E  AD 38 0E  LDA  $0E38
 0E21  ED 36 0E  SBC  $0E36
 0E24  90 09     BCC  $0E2F
 0E26  8D 38 0E  STA  $0E38
 0E29  8C 37 0E  STY  $0E37
 0E2C  EE 33 0E  INC  $0E33
 0E2F  CA        DEX  
 0E30  D0 D8     BNE  $0E0A
 0E32  60        RTS  
 0E33  34 12 0A 00 00 00                                |4.....          |
"
        .parse::<Image>()?;

        let load = image.load().expect("Must be set");
        assert_eq!(0x0e00, load);
        let bus = Bus::default_with_image(&image)?;
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, irq_channel.rx);

        let irq_addr = bus.load_irq_unsafe();
        cpu.bus.store(irq_addr, 0xea);

        cpu.reg.pc = load.wrapping_add(2);
        loop {
            cpu.step_no_spin();
            if p_get!(cpu.reg, I) {
                break;
            }
        }
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
        const RETURN_ADDR: u16 = 0x1234;

        let monitor: Option<Box<dyn Monitor>> = if trace {
            Some(Box::new(TracingMonitor::default()))
        } else {
            None
        };

        let image = input.parse::<Image>()?;
        let bus = Bus::default_with_image(&image)?;
        let irq_channel = IrqChannel::new();
        let mut cpu = Cpu::new(bus.view(), monitor, irq_channel.rx);

        let irq_addr = bus.load_irq_unsafe();
        cpu.bus.store(irq_addr, 0xea);

        cpu.reg.pc = image.start().unwrap_or_default();

        let rts = MOS_6502
            .get_op_info(&Opcode::Rts)
            .expect("RTS must exist")
            .clone();

        cpu.push_word(RETURN_ADDR.wrapping_sub(1));
        p_set!(cpu.reg, B, false);

        let mut result = String::new();
        loop {
            loop {
                cpu.step_no_spin();
                if p_get!(cpu.reg, I) {
                    break;
                }
            }

            match get_brk_addr(&cpu) {
                Some(0xffc0) => break,
                Some(addr) if addr == RETURN_ADDR => break,
                Some(0xffee) => {
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
