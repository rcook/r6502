use crate::single_step_tests::Scenario;
use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

//const SKIPPED_SCENARIO_NAMES: [&str; 1] = ["a9 f0 33"];
const SKIPPED_SCENARIO_NAMES: [&str; 0] = [];

pub(crate) struct ScenarioConfig {
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) scenario_name: Option<String>,
    pub(crate) skipped_scenario_names: Vec<String>,
}

impl ScenarioConfig {
    pub(crate) fn new(filter: &Option<String>) -> Result<Self> {
        let scenario_dir = Self::scenario_dir()?;

        match filter {
            Some(s) => match s.split_once(',') {
                Some((file_name, name)) => Ok(Self {
                    paths: vec![scenario_dir.join(file_name)],
                    scenario_name: Some(String::from(name)),
                    skipped_scenario_names: Vec::new(),
                }),
                None => Ok(Self {
                    paths: vec![scenario_dir.join(s)],
                    scenario_name: None,
                    skipped_scenario_names: SKIPPED_SCENARIO_NAMES
                        .iter()
                        .map(|n| String::from(*n))
                        .collect(),
                }),
            },
            None => {
                let mut paths = Vec::new();
                for p in read_dir(&scenario_dir)? {
                    let p = p?;
                    if p.path().extension().and_then(OsStr::to_str) == Some("json") {
                        paths.push(p.path());
                    }
                }
                Ok(Self {
                    paths,
                    scenario_name: None,
                    skipped_scenario_names: SKIPPED_SCENARIO_NAMES
                        .iter()
                        .map(|n| String::from(*n))
                        .collect(),
                })
            }
        }
    }

    pub(crate) fn read_scenario_file(&self, path: &Path) -> Result<Vec<(String, Scenario)>> {
        let file = File::open(path)?;
        let file_name = path
            .file_name()
            .ok_or_else(|| anyhow!("Cannot get file name from {}", path.display()))?
            .to_str()
            .ok_or_else(|| anyhow!("Cannot get file name from path {}", path.display()))?;
        let scenarios = serde_json::from_reader::<_, Vec<Scenario>>(file)?;
        Ok(scenarios
            .into_iter()
            .filter_map(|s| match &self.scenario_name {
                Some(n) => {
                    if s.name == *n {
                        Some((String::from(file_name), s))
                    } else {
                        None
                    }
                }
                None => {
                    if self.skipped_scenario_names.contains(&s.name) {
                        None
                    } else {
                        Some((String::from(file_name), s))
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
