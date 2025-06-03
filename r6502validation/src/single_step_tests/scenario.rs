use crate::single_step_tests::{Cycle, State};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize)]
pub(crate) struct Scenario {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "initial")]
    pub(crate) initial: State,

    #[serde(rename = "final")]
    pub(crate) r#final: State,

    #[allow(unused)]
    #[serde(rename = "cycles")]
    pub(crate) cycles: Vec<Cycle>,
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Scenario: {}", self.name)?;
        write!(f, "Initial:\n{}", self.initial)?;
        write!(f, "Final:\n{}", self.r#final)
    }
}
