use crate::emulator::OutputDevice;
use anyhow::Result;
use std::io::{stdout, Write};

pub struct TerminalOutput;

impl OutputDevice for TerminalOutput {
    fn write(&self, value: u8) -> Result<()> {
        let mut stdout = stdout();
        if value == 0x0a {
            stdout.write_all(&[13, 10])?;
        } else {
            stdout.write_all(&[value])?;
        }
        stdout.flush()?;
        Ok(())
    }
}
