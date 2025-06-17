use anyhow::Result;
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct RawMode;

impl RawMode {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("Must succeed");
    }
}
