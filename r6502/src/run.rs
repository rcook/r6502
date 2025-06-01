#![allow(unused)]
use crate::{
    Args, DebugMessage, ImageSource, IoMessage, MonitorMessage, Status, SymbolInfo, TestHost, Ui,
    UiHost, VmHost, VmStatus,
};
use anyhow::Result;
use clap::Parser;
use r6502lib::{
    p_set, DummyMonitor, Image, InstructionInfo, Monitor, OpInfo, Opcode, Os, OsBuilder, Reg,
    TotalCycles, TracingMonitor, Vm, VmBuilder, MOS_6502, OSHALT, OSWRCH,
};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    if args.debug {
        run_ui_host(&args);
    } else {
        run_cli_host(&args)?;
    }
    Ok(())
}

fn run_ui_host(args: &Args) -> Result<()> {
    fn run_vm(
        image: Image,
        debug_rx: Receiver<DebugMessage>,
        monitor_tx: Sender<MonitorMessage>,
        io_tx: Sender<IoMessage>,
    ) -> Result<VmStatus> {
        struct HackyMonitor {
            monitor_tx: Sender<MonitorMessage>,
        }

        impl Monitor for HackyMonitor {
            fn on_before_execute(
                &self,
                total_cycles: TotalCycles,
                reg: Reg,
                instruction_info: InstructionInfo,
            ) {
                self.monitor_tx
                    .send(MonitorMessage::BeforeExecute {
                        total_cycles,
                        reg,
                        instruction_info,
                    })
                    .expect("Must succeed")
            }

            fn on_after_execute(
                &self,
                total_cycles: TotalCycles,
                reg: Reg,
                instruction_info: InstructionInfo,
            ) {
                self.monitor_tx
                    .send(MonitorMessage::AfterExecute {
                        total_cycles,
                        reg,
                        instruction_info,
                    })
                    .expect("Must succeed")
            }
        }

        let hacky_monitor = Box::new(HackyMonitor {
            monitor_tx: monitor_tx.clone(),
        });

        let mut vm = VmBuilder::default().monitor(hacky_monitor).build()?;
        let (os, rts) = initialize_vm(&mut vm, &image)?;

        let ui_host = UiHost::new(debug_rx, monitor_tx);
        let mut free_running = false;
        loop {
            while vm.step() {
                let result = ui_host.poll(&vm.s.memory, free_running);
                free_running = result.free_running;
                if !result.is_active {
                    // Handle disconnection
                    return Ok(VmStatus::Disconnected);
                }
            }

            match os.is_os_vector_brk(&vm) {
                Some(OSHALT) => {
                    ui_host.report_status(Status::Halted);
                    return Ok(VmStatus::Halted);
                }
                Some(OSWRCH) => {
                    io_tx.send(IoMessage::WriteChar(vm.s.reg.a as char));
                    os.return_from_os_vector_brk(&mut vm, &rts);
                }
                _ => todo!(),
            }
        }
    }

    let image = Image::load(&args.path, args.origin, args.start)?;
    let symbols = SymbolInfo::load(&args.path)?;
    let debug_channel = channel();
    let monitor_channel = channel();
    let io_channel = channel();
    let mut ui = Ui::new(monitor_channel.1, io_channel.1, debug_channel.0, symbols)?;
    spawn(move || run_vm(image, debug_channel.1, monitor_channel.0, io_channel.0));
    ui.run();
    Ok(())
}

fn run_cli_host(args: &Args) -> Result<()> {
    let monitor: Box<dyn Monitor> = if args.trace {
        Box::new(TracingMonitor)
    } else {
        Box::new(DummyMonitor)
    };

    let mut vm = VmBuilder::default().monitor(monitor).build()?;

    let image = Image::load(&args.path, args.origin, args.start)?;
    let (os, rts) = initialize_vm(&mut vm, &image)?;

    loop {
        while vm.step() {}

        match os.is_os_vector_brk(&vm) {
            Some(OSHALT) => {
                break;
            }
            Some(OSWRCH) => {
                print!("{}", vm.s.reg.a as char);
                os.return_from_os_vector_brk(&mut vm, &rts);
            }
            _ => todo!(),
        }
    }

    Ok(())
}

fn initialize_vm(vm: &mut Vm, image: &Image) -> Result<(Os, OpInfo)> {
    let os = OsBuilder::default().build()?;

    let rts = MOS_6502
        .get_op_info(&Opcode::Rts)
        .expect("RTS must exist")
        .clone();

    os.initialize(&mut vm.s.memory);
    vm.s.memory.load(image);
    vm.s.push_word(OSHALT - 1);
    vm.s.reg.pc = image.start;

    Ok((os, rts))
}

/*
#[cfg(test)]
mod tests {
    use crate::{ImageSource, Status, TestHost, VmStatus};
    use anyhow::Result;

    #[test]
    fn basics0() -> Result<()> {
        let bytes = include_bytes!("../../examples/hello-world.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("HELLO, WORLD!", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(516, result.cycles);
        Ok(())
    }

    #[test]
    fn basics1() -> Result<()> {
        let bytes = include_bytes!("../../examples/strings.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("String0\nString1\n", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(833, result.cycles);
        Ok(())
    }

    #[test]
    fn basics2() -> Result<()> {
        let bytes = include_bytes!("../../examples/test.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("Hello, world\r\n", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(584, result.cycles);
        Ok(())
    }

    // TBD: This program does not seem to terminate
    //#[test]
    fn basics3() -> Result<()> {
        let bytes = include_bytes!("../../examples/randfill.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("Hello, world\r\n", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(584, result.cycles);
        Ok(())
    }

    //#[test]
    fn add8() -> Result<()> {
        let bytes = include_bytes!("../../examples/add8.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(27, result.cycles);
        assert_eq!(0x46, result.machine_state.memory[0x0e00]);
        Ok(())
    }

    //#[test]
    fn add16() -> Result<()> {
        let bytes = include_bytes!("../../examples/add16.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(39, result.cycles);
        assert_eq!(0x68, result.machine_state.memory[0x0e00]);
        assert_eq!(0xac, result.machine_state.memory[0x0e01]);
        Ok(())
    }

    //#[test]
    fn div16() -> Result<()> {
        let bytes = include_bytes!("../../examples/div16.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("", stdout);
        assert_eq!(VmStatus::Halted, result.status);
        assert_eq!(758, result.cycles);
        const NUM1: usize = 0x0026;
        const REM: usize = 0x002a;
        let quotient_lo = result.machine_state.memory[NUM1];
        let quotient_hi = result.machine_state.memory[NUM1 + 1];
        let remainder_lo = result.machine_state.memory[REM];
        let remainder_hi = result.machine_state.memory[REM + 1];
        assert_eq!(0xd2, quotient_lo);
        assert_eq!(0x01, quotient_hi);
        assert_eq!(0x00, remainder_lo);
        assert_eq!(0x00, remainder_hi);
        Ok(())
    }

    fn run(bytes: &[u8]) -> Result<(String, VmStatus)> {
        let test_host = TestHost::new();
        let result = run_vm(
            &test_host,
            Some(ImageSource::from_bytes(bytes, None, None)),
            false,
        )?;
        Ok((test_host.stdout(), result))
    }
}
*/
