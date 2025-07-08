use crate::emulator::OutputDevice;
use crate::machine_config::CharSet;
use crate::terminal::{RawMode, VduCode, VDU_CODES_BY_CODE};
use anyhow::Result;
use std::io::{stdout, Stdout, Write};

pub struct VduDriver {
    code: Option<&'static VduCode>,
    args: Vec<u8>,
}

impl VduDriver {
    fn process(&mut self, value: u8, stdout: &mut Stdout) -> Result<Option<u8>> {
        if let Some(temp) = self.code {
            self.args.push(value);
            let count = self.args.len();
            let arg_count = usize::from(temp.2);
            assert!(count <= arg_count);
            if count == arg_count {
                let s = self
                    .args
                    .drain(0..)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let raw_mode = RawMode::disable()?;
                println!(
                    "[VDU {code}, {args} ({description}) unimplemented]",
                    code = temp.0,
                    args = s,
                    description = temp.3
                );
                drop(raw_mode);
                self.code = None;
                self.args.clear();
            }
            return Ok(None);
        }

        match VDU_CODES_BY_CODE.get(&value) {
            Some((_, _, 0, _, Some(f))) => {
                f(stdout);
                Ok(None)
            }
            Some((_, _, 0, _, None)) | None => Ok(Some(value)),
            Some(code) => {
                self.code = Some(code);
                Ok(None)
            }
        }
    }
}

impl Default for VduDriver {
    fn default() -> Self {
        Self {
            code: None,
            args: Vec::with_capacity(9),
        }
    }
}

impl OutputDevice for VduDriver {
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()> {
        let mut stdout = stdout();

        let value = match char_set {
            CharSet::Acorn => match self.process(value, &mut stdout)? {
                Some(value) => value,
                None => return Ok(()),
            },
            _ => value,
        };

        if let Some(value) = char_set.translate_out(value) {
            match value {
                0x0a => stdout.write_all(&[13, 10])?,
                _ => stdout.write_all(&[value])?,
            }
            stdout.flush()?;
        }

        Ok(())
    }
}
