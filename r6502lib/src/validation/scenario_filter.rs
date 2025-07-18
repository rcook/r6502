use crate::validation::ScenarioLoader;
use anyhow::{Result, bail};
use r6502validation::Scenario;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const SKIPPED_SCENARIO_NAMES: [&str; 0] = [];

pub struct ScenarioFilter {
    pub paths: Vec<PathBuf>,
    pub scenario_name: Option<String>,
    pub skipped_scenario_names: Vec<String>,
}

impl ScenarioFilter {
    pub fn new(loader: &ScenarioLoader, filter: &Option<String>) -> Result<Self> {
        let Some(s) = filter else {
            let paths = Self::get_scenario_files(&loader.json_dir, "json")?;
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
                paths: vec![loader.json_dir.join(format!("{opcode:02x}.json"))],
                scenario_name: None,
                skipped_scenario_names: SKIPPED_SCENARIO_NAMES
                    .iter()
                    .map(|n| String::from(*n))
                    .collect(),
            });
        };

        let opcode = u8::from_str_radix(opcode_value, 16)?;

        Ok(Self {
            paths: vec![loader.json_dir.join(format!("{opcode:02x}.json"))],
            scenario_name: Some(String::from(s)),
            skipped_scenario_names: Vec::new(),
        })
    }

    #[must_use]
    pub fn filter(&self, scenarios: Vec<Scenario>) -> Vec<Scenario> {
        scenarios
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
            .collect()
    }

    fn get_scenario_files(dir: &Path, ext: &str) -> Result<Vec<PathBuf>> {
        let d = match read_dir(dir) {
            Ok(d) => d,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                bail!("no such directory {dir}", dir = dir.display())
            }
            Err(e) => bail!(e),
        };

        let mut paths = Vec::new();
        for p in d {
            let p = p?;
            if p.path().extension().and_then(OsStr::to_str) == Some(ext) {
                paths.push(p.path());
            }
        }
        paths.sort();

        Ok(paths)
    }
}
