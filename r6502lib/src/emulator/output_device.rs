use crate::machine_config::CharSet;
use anyhow::Result;

pub trait OutputDevice: Send + 'static {
    fn write(&mut self, char_set: &CharSet, value: u8) -> Result<()>;
}
