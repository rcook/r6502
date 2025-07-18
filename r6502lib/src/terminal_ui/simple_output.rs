use crate::emulator::OutputDevice;
use crate::machine_config::CharSet;
use anyhow::Result;
use std::io::{Write, stdout};

pub struct SimpleOutput;

impl OutputDevice for SimpleOutput {
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()> {
        if let Some(value) = char_set.translate_out(value) {
            let mut stdout = stdout();
            match value {
                0x0a => stdout.write_all(&[13, 10])?,
                _ => stdout.write_all(&[value])?,
            }
            stdout.flush()?;
        }

        Ok(())
    }
}
