use crate::VMHost;

pub(crate) struct CliHost;

impl CliHost {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl VMHost for CliHost {
    fn write_stdout(&self, c: char) {
        print!("{c}")
    }
}
