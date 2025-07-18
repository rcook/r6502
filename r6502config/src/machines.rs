use crate::Machine;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Machines {
    #[serde(rename = "defaultMachine")]
    pub default_machine: String,

    #[serde(rename = "machines")]
    pub machines: Vec<Machine>,
}
