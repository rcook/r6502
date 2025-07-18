use crate::Scenario;
use anyhow::{Result, anyhow, bail};
use dirs::config_dir;
use std::env::current_dir;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct ScenarioLoader {
    pub json_dir: PathBuf,
    pub archive_dir: PathBuf,
}

impl ScenarioLoader {
    pub fn new() -> Result<Self> {
        let json_dir = Self::strip_parents(&Self::current_source_path()?, 4)?
            .join("SingleStepTests-65x02")
            .join("6502")
            .join("v1");
        if !json_dir.is_dir() {
            bail!(
                "path {json_dir} is not a directory or does not exist",
                json_dir = json_dir.display()
            )
        }

        let archive_dir = config_dir()
            .ok_or_else(|| anyhow!("could not get configuration directory"))?
            .join("r6502")
            .join("scenarios");
        if !archive_dir.is_dir() {
            bail!(
                "path {archive_dir} is not a directory or does not exist",
                archive_dir = archive_dir.display()
            )
        }

        Ok(Self {
            json_dir,
            archive_dir,
        })
    }

    pub fn read_scenarios(&self, json_path: &Path) -> Result<Vec<Scenario>> {
        let archive_path = self.get_archive_path(json_path)?;
        if archive_path.is_file() {
            return Self::read_archive(&archive_path);
        }

        let scenarios = Self::read_json(json_path)?;
        Self::write_archive(&archive_path, &scenarios)?;
        Ok(scenarios)
    }

    fn current_source_path() -> Result<PathBuf> {
        Ok(current_dir()?.join(file!()))
    }

    fn strip_parents(path: &Path, n: i32) -> Result<&Path> {
        let mut temp = path;
        for _ in 0..n {
            temp = temp.parent().ok_or_else(|| anyhow!("parent must exist"))?;
        }
        Ok(temp)
    }

    fn read_json(json_path: &Path) -> Result<Vec<Scenario>> {
        let file = File::open(json_path)?;
        serde_json::from_reader(file).map_err(|e| anyhow!(e))
    }

    fn read_archive(archive_path: &Path) -> Result<Vec<Scenario>> {
        let mut file = File::open(archive_path)?;
        let mut bytes = Vec::new();
        _ = file.read_to_end(&mut bytes)?;
        Ok(rkyv::from_bytes::<_, rancor::Error>(&bytes)?)
    }

    fn write_archive(archive_path: &Path, scenarios: &Vec<Scenario>) -> Result<()> {
        create_dir_all(
            archive_path
                .parent()
                .ok_or_else(|| anyhow!("cannot get directory"))?,
        )?;
        let bytes = rkyv::to_bytes::<rancor::Error>(scenarios)?;
        let mut file = File::create(archive_path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    fn get_archive_path(&self, json_path: &Path) -> Result<PathBuf> {
        let file_name = json_path
            .file_name()
            .ok_or_else(|| anyhow!("could not extract file name"))?;
        let mut archive_path = self.archive_dir.join(file_name);
        archive_path.set_extension("rkyv");
        Ok(archive_path)
    }
}
