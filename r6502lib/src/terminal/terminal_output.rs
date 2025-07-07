use crate::emulator::{OutputDevice, VduDriver};
use crate::machine_config::CharSet;
use anyhow::Result;
use std::io::{stdout, Write};

#[derive(Default)]
pub struct TerminalOutput {
    vdu_driver: VduDriver,
}

impl OutputDevice for TerminalOutput {
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()> {
        let value = match char_set {
            CharSet::Acorn => match self.vdu_driver.process(value)? {
                Some(value) => value,
                None => return Ok(()),
            },
            _ => value,
        };

        if let Some(value) = char_set.translate_out(value) {
            let mut stdout = stdout();
            match value {
                0x0a => stdout.write_all(&[13, 10])?,
                127 => stdout.write_all(&[65, 66])?,
                _ => stdout.write_all(&[value])?,
            }
            stdout.flush()?;
        }

        Ok(())
    }
}
