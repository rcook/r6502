use crate::single_step_tests::{Cycle, State};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct TestCase {
    #[serde(rename = "name")]
    pub(crate) name: String,
    #[serde(rename = "initial")]
    pub(crate) initial: State,
    #[serde(rename = "final")]
    pub(crate) r#final: State,
    #[serde(rename = "cycles")]
    pub(crate) cycles: Vec<Cycle>,
}
