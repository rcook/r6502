use crate::single_step_tests::Scenario;
use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

const SKIPPED_SCENARIO_NAMES: [&str; 1] = ["a9 f0 33"];

pub(crate) struct ScenarioConfig {
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) scenario_name: Option<String>,
    pub(crate) skipped_scenario_names: Vec<String>,
}

impl ScenarioConfig {
    pub(crate) fn new(filter: &Option<String>) -> Result<Self> {
        let scenario_dir = Self::scenario_dir()?;

        let Some(s) = filter else {
            let mut paths = Vec::new();
            for p in read_dir(&scenario_dir)? {
                let p = p?;
                if p.path().extension().and_then(OsStr::to_str) == Some("json") {
                    paths.push(p.path());
                }
            }
            return Ok(Self {
                paths,
                scenario_name: None,
                skipped_scenario_names: SKIPPED_SCENARIO_NAMES
                    .iter()
                    .map(|n| String::from(*n))
                    .collect(),
            });
        };

        let Some((opcode_value, _)) = s.split_once(' ') else {
            let opcode = u8::from_str_radix(s, 16)?;
            return Ok(Self {
                paths: vec![scenario_dir.join(format!("{opcode:02x}.json"))],
                scenario_name: None,
                skipped_scenario_names: SKIPPED_SCENARIO_NAMES
                    .iter()
                    .map(|n| String::from(*n))
                    .collect(),
            });
        };

        let opcode = u8::from_str_radix(opcode_value, 16)?;

        Ok(Self {
            paths: vec![scenario_dir.join(format!("{opcode:02x}.json"))],
            scenario_name: Some(String::from(s)),
            skipped_scenario_names: Vec::new(),
        })
    }

    pub(crate) fn read_scenarios(&self, path: &Path) -> Result<Vec<Scenario>> {
        println!("Reading scenarios from {}", path.display());
        let file = File::open(path)?;
        let scenarios = serde_json::from_reader::<_, Vec<Scenario>>(file)?;
        Ok(scenarios
            .into_iter()
            .filter_map(|s| match &self.scenario_name {
                Some(n) => {
                    if s.name == *n {
                        Some(s)
                    } else {
                        None
                    }
                }
                None => {
                    if self.skipped_scenario_names.contains(&s.name) {
                        None
                    } else {
                        Some(s)
                    }
                }
            })
            .collect())
    }

    fn strip_parents(path: &Path, n: i32) -> Result<&Path> {
        let mut temp = path;
        for _ in 0..n {
            temp = temp.parent().ok_or_else(|| anyhow!("Parent must exist"))?
        }
        Ok(temp)
    }

    fn current_source_path() -> Result<PathBuf> {
        Ok(current_dir()?.join(file!()))
    }

    fn scenario_dir() -> Result<PathBuf> {
        Ok(Self::strip_parents(&Self::current_source_path()?, 4)?
            .join("SingleStepTests-65x02/6502/v1"))
    }
}
