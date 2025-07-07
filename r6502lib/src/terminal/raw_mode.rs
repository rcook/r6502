use anyhow::Result;
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct RawMode(bool);

impl RawMode {
    pub fn enable() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self(true))
    }

    pub fn disable() -> Result<Self> {
        disable_raw_mode()?;
        Ok(Self(false))
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        if self.0 {
            _ = disable_raw_mode();
        } else {
            _ = enable_raw_mode();
        }
    }
}
