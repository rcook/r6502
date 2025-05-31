#![allow(unused)]
use crate::{run_vm, Args, ImageSource, TestHost, UIHost, UI};
use anyhow::Result;
use clap::Parser;
use r6502lib::{
    p_set, DummyMonitor, Image, Monitor, Opcode, OsBuilder, TracingMonitor, Vm, VmBuilder, OSHALT,
    OSWRCH,
};
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    let image = Image::load(&args.path, args.origin, args.start)?;
    if args.debug {
        /*
        let debug_channel = channel();
        let status_channel = channel();
        let symbols = match image_source.as_ref() {
            Some(image_source) => image_source.load_symbols()?,
            None => Vec::new(),
        };
        let mut ui = UI::new(status_channel.1, debug_channel.0, symbols)?;
        let ui_host = UIHost::new(debug_channel.1, status_channel.0);
        spawn(move || {
            run_vm(&ui_host, image_source, !args.debug).expect("Must succeed");
        });
        ui.run();
        */
        todo!();
    } else {
        run_cli_host(&image, args.trace)?;
    }
    Ok(())
}

fn run_cli_host(image: &Image, trace: bool) -> Result<()> {
    let monitor: Box<dyn Monitor> = if trace {
        Box::new(TracingMonitor)
    } else {
        Box::new(DummyMonitor)
    };

    let mut vm = VmBuilder::default().monitor(monitor).build()?;
    let os = OsBuilder::default().build()?;

    let rts = vm
        .cpu
        .get_op_info(&Opcode::Rts)
        .expect("RTS must exist")
        .clone();

    os.initialize(&mut vm.s.memory);
    vm.s.memory.load(image);
    vm.s.push_word(OSHALT - 1);
    vm.s.reg.pc = image.start;

    loop {
        while vm.step() {}

        match os.is_os_vector_brk(&vm) {
            Some(OSHALT) => {
                break;
            }
            Some(OSWRCH) => {
                print!("{}", vm.s.reg.a as char);
                vm.s.pull(); // Is this P?
                vm.s.pull_word(); // What's this?
                p_set!(vm.s.reg, B, false);
                rts.op.execute_no_operand(&mut vm.s);
            }
            _ => todo!(),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{run_vm, ImageSource, RunVMResult, RunVMStatus, Status, TestHost};
    use anyhow::Result;

    #[test]
    fn basics0() -> Result<()> {
        let bytes = include_bytes!("../../examples/hello-world.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("HELLO, WORLD!", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(516, result.cycles);
        Ok(())
    }

    #[test]
    fn basics1() -> Result<()> {
        let bytes = include_bytes!("../../examples/strings.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("String0\nString1\n", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(833, result.cycles);
        Ok(())
    }

    #[test]
    fn basics2() -> Result<()> {
        let bytes = include_bytes!("../../examples/test.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("Hello, world\r\n", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(584, result.cycles);
        Ok(())
    }

    // TBD: This program does not seem to terminate
    //#[test]
    fn basics3() -> Result<()> {
        let bytes = include_bytes!("../../examples/randfill.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("Hello, world\r\n", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(584, result.cycles);
        Ok(())
    }

    //#[test]
    fn add8() -> Result<()> {
        let bytes = include_bytes!("../../examples/add8.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(27, result.cycles);
        assert_eq!(0x46, result.machine_state.memory[0x0e00]);
        Ok(())
    }

    //#[test]
    fn add16() -> Result<()> {
        let bytes = include_bytes!("../../examples/add16.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
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
        assert_eq!(RunVMStatus::Halted, result.status);
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

    fn run(bytes: &[u8]) -> Result<(String, RunVMResult)> {
        let test_host = TestHost::new();
        let result = run_vm(
            &test_host,
            Some(ImageSource::from_bytes(bytes, None, None)),
            false,
        )?;
        Ok((test_host.stdout(), result))
    }
}
