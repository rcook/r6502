#![allow(unused)]
use crate::{run_vm, Args, CliHost, ImageSource, TestHost, UIHost, UI};
use anyhow::Result;
use clap::Parser;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    let image_source = Some(ImageSource::from_file(&args.path, args.origin, args.start));
    if args.debug {
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
    } else {
        let cli_host = CliHost::new();
        run_vm(&cli_host, image_source, !args.debug)?;
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

    #[test]
    fn add8() -> Result<()> {
        let bytes = include_bytes!("../../examples/add8.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(27, result.cycles);
        assert_eq!(0x46, result.machine_state.memory[0x0e00]);
        Ok(())
    }

    #[test]
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
