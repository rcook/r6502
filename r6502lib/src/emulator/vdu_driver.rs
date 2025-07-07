use crate::emulator::{VduCode, VDU_CODES_BY_CODE};
use crate::terminal::RawMode;
use anyhow::Result;

pub struct VduDriver {
    code: Option<&'static VduCode>,
    args: Vec<u8>,
}

impl Default for VduDriver {
    fn default() -> Self {
        Self {
            code: None,
            args: Vec::with_capacity(9),
        }
    }
}

impl VduDriver {
    pub fn process(&mut self, value: u8) -> Result<Option<u8>> {
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
            Some((_, _, 0, _)) | None => Ok(Some(value)),
            Some(code) => {
                self.code = Some(code);
                Ok(None)
            }
        }
    }
}
