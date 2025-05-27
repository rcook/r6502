#![allow(unused)]
use crate::{run_vm, Args, CliHost, ImageInfo, TestHost, UIHost, UI};
use anyhow::Result;
use clap::Parser;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    let image_info = Some(ImageInfo::from_file(&args.path, args.origin, args.start));
    if args.debug {
        let debug_channel = channel();
        let status_channel = channel();
        let mut ui = UI::new(status_channel.1, debug_channel.0)?;
        let ui_host = UIHost::new(debug_channel.1, status_channel.0);
        spawn(move || {
            run_vm(&ui_host, image_info, !args.debug).expect("Must succeed");
        });
        ui.run();
    } else {
        let cli_host = CliHost::new();
        run_vm(&cli_host, image_info, !args.debug)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{run_vm, ImageInfo, RunVMResult, RunVMStatus, Status, TestHost};
    use anyhow::Result;

    #[test]
    fn basics0() -> Result<()> {
        let bytes = include_bytes!("../examples/hello-world.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("HELLO, WORLD!", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(516, result.cycles);
        Ok(())
    }

    #[test]
    fn basics1() -> Result<()> {
        let bytes = include_bytes!("../examples/strings.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("String0\nString1\n", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(833, result.cycles);
        Ok(())
    }

    #[test]
    fn basics2() -> Result<()> {
        let bytes = include_bytes!("../examples/test.r6502");
        let (stdout, result) = run(bytes)?;
        assert_eq!("Hello, world\r\n", stdout);
        assert_eq!(RunVMStatus::Halted, result.status);
        assert_eq!(584, result.cycles);
        Ok(())
    }

    fn run(bytes: &[u8]) -> Result<(String, RunVMResult)> {
        let test_host = TestHost::new();
        let result = run_vm(
            &test_host,
            Some(ImageInfo::from_bytes(bytes, None, None)),
            false,
        )?;
        Ok((test_host.stdout(), result))
    }
}
