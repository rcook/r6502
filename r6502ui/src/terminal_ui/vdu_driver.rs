use crate::terminal_ui::{RawMode, VDU_CODES_BY_CODE, VduCode};
use anyhow::Result;
use r6502config::CharSet;
use r6502core::emulator::OutputDevice;
use r6502core::emulator::char_set_util::translate_out;
use std::io::{Stdout, Write, stdout};

pub struct VduDriver {
    code: Option<&'static VduCode>,
    args: Vec<u8>,
}

impl VduDriver {
    fn process(&mut self, value: u8, stdout: &mut Stdout) -> Result<Option<u8>> {
        fn show_unimplemented(code: &VduCode, args: &mut Vec<u8>) -> Result<()> {
            let s = args
                .drain(0..)
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let raw_mode = RawMode::disable()?;
            println!(
                "[VDU {code}, {args} ({description}) unimplemented]",
                code = code.0,
                args = s,
                description = code.3
            );
            drop(raw_mode);
            Ok(())
        }
        if let Some(temp) = self.code {
            self.args.push(value);
            let count = self.args.len();
            let arg_count = usize::from(temp.2);
            assert!(count <= arg_count);
            if count == arg_count {
                match temp.4 {
                    Some(f) => f(stdout, &self.args),
                    None => show_unimplemented(temp, &mut self.args)?,
                }
                self.code = None;
                self.args.clear();
            }
            return Ok(None);
        }

        match VDU_CODES_BY_CODE.get(&value) {
            Some((_, _, 0, _, Some(f))) => {
                f(stdout, &self.args);
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

        if let Some(value) = translate_out(char_set, value) {
            match value {
                0x0a => stdout.write_all(&[13, 10])?,
                _ => stdout.write_all(&[value])?,
            }
            stdout.flush()?;
        }

        Ok(())
    }
}
