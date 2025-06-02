#[cfg(test)]
mod tests {
    use crate::single_step_tests::TestCase;
    use anyhow::Result;
    use std::fs::File;

    //#[test]
    fn basics() -> Result<()> {
        let file = File::open("sample.json")?;
        _ = serde_json::from_reader::<_, TestCase>(file)?;
        Ok(())
    }
}
