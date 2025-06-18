use anyhow::Result;

pub trait OutputDevice: Send + 'static {
    fn write(&self, value: u8) -> Result<()>;
}
