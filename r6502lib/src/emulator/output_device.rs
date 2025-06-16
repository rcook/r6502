use anyhow::Result;

pub trait OutputDevice: Send + 'static {
    fn dup(&self) -> Box<dyn OutputDevice>;
    fn write(&self, ch: char) -> Result<()>;
}
