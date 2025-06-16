use anyhow::Result;

pub trait OutputDevice: Send + 'static {
    fn write(&self, ch: char) -> Result<()>;
}
