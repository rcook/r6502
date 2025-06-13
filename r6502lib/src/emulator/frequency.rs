use std::time::Duration;

pub enum Frequency {
    MHz(u64),
}

impl Frequency {
    #[must_use]
    pub fn get_tick(&self) -> Duration {
        match self {
            Self::MHz(value) => Duration::from_nanos(1_000_000_000 / value),
        }
    }
}
