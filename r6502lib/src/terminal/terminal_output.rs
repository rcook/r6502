use crate::emulator::OutputDevice;
use anyhow::Result;
use std::io::{stdout, Write};

pub struct TerminalOutput;

impl OutputDevice for TerminalOutput {
    fn write(&self, ch: char) -> Result<()> {
        let mut stdout = stdout();
        if ch == '\n' {
            stdout.write_all(&[13, 10])?;
        } else {
            stdout.write_all(&[ch as u8])?;
        }
        stdout.flush()?;
        Ok(())
    }
}
