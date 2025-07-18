use anyhow::{Result, anyhow};
use r6502validation::Scenario;

pub fn from_json(s: &str) -> Result<Scenario> {
    serde_json::from_str(s).map_err(|e| anyhow!(e))
}
