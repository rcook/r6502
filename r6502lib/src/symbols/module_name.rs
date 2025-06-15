use anyhow::{Error, Result};
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ModuleName {
    pub name: String,
    pub path: Option<PathBuf>,
}

impl FromStr for ModuleName {
    type Err = Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        match s.strip_suffix(')') {
            Some(prefix) => match prefix.rsplit_once('(') {
                Some((prefix, suffix)) => Ok(Self {
                    name: String::from(suffix),
                    path: Some(prefix.parse()?),
                }),
                None => Ok(Self {
                    name: String::from(s),
                    path: None,
                }),
            },
            None => Ok(Self {
                name: String::from(s),
                path: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::symbols::ModuleName;
    use anyhow::Result;
    use rstest::rstest;
    use std::path::{Path, PathBuf};

    #[rstest]
    #[case("a1basic.o", None, "a1basic.o")]
    #[case(
        "copydata.o",
        Some("C:\\bin\\cc65\\lib\\none.lib".parse().unwrap()),
        "C:\\bin\\cc65\\lib/none.lib(copydata.o)"
    )]
    fn basics(
        #[case] expected_name: &str,
        #[case] expected_path: Option<PathBuf>,
        #[case] input: &str,
    ) -> Result<()> {
        let result = input.parse::<ModuleName>()?;
        assert_eq!(expected_name, result.name);
        assert_eq!(expected_path, result.path);
        Ok(())
    }
}
