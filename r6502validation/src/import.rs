use anyhow::{anyhow, Result};
use r6502validationlib::Scenario;
use std::ffi::OsStr;
use std::fs::{create_dir_all, read_dir};
use std::path::Path;

pub(crate) fn import(input_dir: &Path, output_dir: &Path) -> Result<()> {
    create_dir_all(output_dir)?;
    for entry in read_dir(input_dir)? {
        let entry = entry?;
        let json_path = entry.path();
        if json_path.extension().and_then(OsStr::to_str) == Some("json") {
            let file_name = json_path
                .file_name()
                .ok_or_else(|| anyhow!("Could not extract file name"))?;
            let mut rkyv_path = output_dir.join(file_name);
            rkyv_path.set_extension("rkyv");
            let scenarios = Scenario::read_all(&json_path)?;
            Scenario::write_rkyv(&rkyv_path, &scenarios)?;
            println!("Written {}", rkyv_path.display())
        }
    }
    Ok(())
}
