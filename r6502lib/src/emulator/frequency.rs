use std::time::Duration;

pub(crate) enum Frequency {
    MHz(u64),
}

impl Frequency {
    pub(crate) fn get_tick(&self) -> Duration {
        match self {
            Self::MHz(value) => Duration::from_nanos(1_000_000_000 / value),
        }
    }
}
