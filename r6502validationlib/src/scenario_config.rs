use crate::{Scenario, ScenarioFormat};
use anyhow::{anyhow, Result};
use dirs::home_dir;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

const SKIPPED_SCENARIO_NAMES: [&str; 1] = ["a9 f0 33"];

pub struct ScenarioConfig {
    pub paths: Vec<PathBuf>,
    pub scenario_name: Option<String>,
    pub skipped_scenario_names: Vec<String>,
}

impl ScenarioConfig {
    pub fn new(format: ScenarioFormat, filter: &Option<String>) -> Result<Self> {
        let (dir, paths, ext) = Self::blah(format)?;

        let Some(s) = filter else {
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
                paths: vec![dir.join(format!("{opcode:02x}.{ext}"))],
                scenario_name: None,
                skipped_scenario_names: SKIPPED_SCENARIO_NAMES
                    .iter()
                    .map(|n| String::from(*n))
                    .collect(),
            });
        };

        let opcode = u8::from_str_radix(opcode_value, 16)?;

        Ok(Self {
            paths: vec![dir.join(format!("{opcode:02x}.{ext}"))],
            scenario_name: Some(String::from(s)),
            skipped_scenario_names: Vec::new(),
        })
    }

    pub fn read_scenarios(&self, path: &Path) -> Result<Vec<Scenario>> {
        println!("Reading scenarios from {}", path.display());
        Ok(Scenario::read_all(path)?
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

    fn blah(format: ScenarioFormat) -> Result<(PathBuf, Vec<PathBuf>, String)> {
        match format {
            ScenarioFormat::Json => {
                let dir = Self::strip_parents(&Self::current_source_path()?, 3)?
                    .join("SingleStepTests-65x02")
                    .join("6502")
                    .join("v1");
                let files = Self::get_scenario_files(&dir, "json")?;
                Ok((dir, files, String::from("json")))
            }
            ScenarioFormat::Rkyv => {
                let dir = home_dir().ok_or_else(|| anyhow!("Could not get home directory"))?;
                let dir = dir.join(".config").join("r6502validation");
                let files = Self::get_scenario_files(&dir, "rkyv")?;
                Ok((dir, files, String::from("rkyv")))
            }
        }
    }

    fn get_scenario_files(dir: &Path, ext: &str) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        for p in read_dir(dir)? {
            let p = p?;
            if p.path().extension().and_then(OsStr::to_str) == Some(ext) {
                paths.push(p.path());
            }
        }
        paths.sort();
        Ok(paths)
    }
}
