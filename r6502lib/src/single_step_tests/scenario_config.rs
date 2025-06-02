use crate::single_step_tests::Scenario;
use anyhow::{anyhow, bail, Result};
use std::env::{current_dir, var, VarError};
use std::ffi::OsStr;
use std::fs::{read_dir, File};
use std::path::{Path, PathBuf};

// R6502LIB_SCENARIOS=1 means "run all scenarios (except skipped ones)"
// R6502LIB_SCENARIOS="a9.json,a9 cc 21" means "run this single scenario (even it's skipped)"
// R6502LIB_SCENARIOS=a9.json means "run this set of scenarios (except skipped ones)"
// Otherwise do nothing since these tests are quite time-consuming
pub(crate) const ENV_NAME: &str = "R6502LIB_SCENARIOS";

pub(crate) struct ScenarioConfig {
    pub(crate) paths: Vec<PathBuf>,
    pub(crate) skipped_scenario_names: Vec<String>,
}

impl ScenarioConfig {
    pub(crate) fn from_env(skipped_scenario_names: &[&str]) -> Result<Self> {
        let scenario_dir = Self::scenario_dir()?;

        Ok(match var(ENV_NAME) {
            Ok(s) => match s.split_once(',') {
                Some((file_name, name)) => Self {
                    paths: vec![scenario_dir.join(file_name)],
                    skipped_scenario_names: Vec::new(),
                },
                None => {
                    let skipped_scenario_names = skipped_scenario_names
                        .iter()
                        .map(|n| String::from(*n))
                        .collect();
                    if s == "1" {
                        let mut paths = Vec::new();
                        for p in read_dir(&scenario_dir)? {
                            let p = p?;
                            if p.path().extension().and_then(OsStr::to_str) == Some("json") {
                                paths.push(p.path());
                            }
                        }
                        Self {
                            paths,
                            skipped_scenario_names,
                        }
                    } else {
                        Self {
                            paths: vec![scenario_dir.join(s)],
                            skipped_scenario_names,
                        }
                    }
                }
            },
            Err(VarError::NotPresent) => Self {
                paths: Vec::new(),
                skipped_scenario_names: vec![],
            },
            Err(e) => bail!(e),
        })
    }

    pub(crate) fn read_scenario_file(&self, path: &Path) -> Result<Vec<(String, Scenario)>> {
        let file = File::open(path)?;
        let scenarios = serde_json::from_reader::<_, Vec<Scenario>>(file)?;
        Ok(scenarios
            .into_iter()
            .filter_map(|s| {
                if self.skipped_scenario_names.contains(&s.name) {
                    None
                } else {
                    Some((String::from("a9.json"), s))
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
        let cwd = current_dir()?;
        let workspace_dir = Self::strip_parents(&cwd, 1)?;
        Ok(workspace_dir.join(file!()))
    }

    fn scenario_dir() -> Result<PathBuf> {
        Ok(Self::strip_parents(&Self::current_source_path()?, 4)?
            .join("SingleStepTests-65x02/6502/v1"))
    }
}
