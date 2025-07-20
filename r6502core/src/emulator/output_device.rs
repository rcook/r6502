use anyhow::Result;
use r6502config::CharSet;

pub trait OutputDevice: Send + 'static {
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()>;
}
