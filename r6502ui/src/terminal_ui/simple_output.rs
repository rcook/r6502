use anyhow::Result;
use r6502config::CharSet;
use r6502lib::emulator::OutputDevice;
use r6502lib::emulator::char_set_util::translate_out;
use std::io::{Write, stdout};

pub struct SimpleOutput;

impl OutputDevice for SimpleOutput {
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()> {
        if let Some(value) = translate_out(char_set, value) {
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
