use crate::{Cycle, State};
use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Deserialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
pub struct Scenario {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "initial")]
    pub initial: State,

    #[serde(rename = "final")]
    pub r#final: State,

    #[serde(rename = "cycles")]
    pub cycles: Vec<Cycle>,
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Scenario: {}", self.name)?;
        write!(f, "Initial:\n{}", self.initial)?;
        write!(f, "Final:\n{}", self.r#final)
    }
}
