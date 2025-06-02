#[cfg(test)]
mod tests {
    use crate::single_step_tests::Scenario;
    use crate::{Vm, P};
    use anyhow::{anyhow, bail, Result};
    use rstest::{fixture, rstest};
    use std::env::{current_dir, var, VarError};
    use std::fs::{read_dir, File};
    use std::panic::catch_unwind;
    use std::path::{absolute, Path, PathBuf};
    use std::str::FromStr;
    use std::sync::{LazyLock, Mutex};

    const SKIPPED_SCENARIO_NAMES: [&str; 1] = ["a9 f0 33"];
    const ENV_NAME: &str = "R6502LIB_SCENARIO";
    static SCENARIO_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        fn strip_parents(path: &Path, n: i32) -> Result<&Path> {
            let mut temp = path;
            for _ in 0..n {
                temp = temp.parent().ok_or_else(|| anyhow!("Parent must exist"))?
            }
            Ok(temp)
        }

        fn this_path() -> Result<PathBuf> {
            let cwd = current_dir()?;
            let workspace_dir = strip_parents(&cwd, 1)?;
            Ok(workspace_dir.join(file!()))
        }

        fn scenario_dir() -> Result<PathBuf> {
            Ok(strip_parents(&this_path()?, 4)?.join("SingleStepTests-65x02/6502/v1"))
        }

        scenario_dir().expect("Must succeed")
    });

    #[test]
    fn scenarios() -> Result<()> {
        let scenarios = read_scenarios()?;
        let mut count = 0;
        let mut failure_count = 0;
        for s in scenarios.iter() {
            match catch_unwind(|| run_scenario(&s.1)) {
                Ok(result) => {
                    if !result {
                        println!(
                            "Scenario \"{}\" failed: set environment variable {ENV_NAME}=\"{},{}\"",
                            s.1.name, s.0, s.1.name
                        );
                        failure_count += 1;
                    }
                }
                Err(e) => {
                    println!(
                        "Scenario \"{}\" failed: set environment variable {ENV_NAME}=\"{},{}\"",
                        s.1.name, s.0, s.1.name
                    );
                    failure_count += 1;
                }
            }

            count += 1;
        }

        if failure_count > 0 {
            panic!("Out of {count} scenarios, {failure_count} failed")
        }

        Ok(())
    }

    fn read_scenarios() -> Result<Vec<(String, Scenario)>> {
        fn read_scenario_file(path: &Path, skip_tests: bool) -> Result<Vec<(String, Scenario)>> {
            let file = File::open(path)?;
            let scenarios = serde_json::from_reader::<_, Vec<Scenario>>(file)?;
            Ok(scenarios
                .into_iter()
                .filter_map(|s| {
                    if skip_tests && SKIPPED_SCENARIO_NAMES.contains(&s.name.as_str()) {
                        None
                    } else {
                        Some((String::from("a9.json"), s))
                    }
                })
                .collect())
        }

        Ok(match var(ENV_NAME) {
            Ok(s) => match s.split_once(',') {
                Some((file_name, name)) => {
                    read_scenario_file(&SCENARIO_DIR.join(file_name), false)?
                        .into_iter()
                        .filter(|s| s.1.name == name)
                        .collect()
                }
                None => read_scenario_file(&SCENARIO_DIR.join(s), true)?,
            },
            Err(VarError::NotPresent) => {
                let mut all_scenarios = Vec::new();
                for d in read_dir(&*SCENARIO_DIR)? {
                    let d = d?;
                    let scenarios = read_scenario_file(&d.path(), true)?;
                    all_scenarios.extend(scenarios);
                }
                todo!();
                all_scenarios
            }
            Err(e) => bail!(e),
        })
    }

    fn run_scenario(scenario: &Scenario) -> bool {
        macro_rules! fail_if_not_eq {
            ($expected: expr, $actual: expr) => {
                if $actual != $expected {
                    return false;
                }
            };
        }

        let mut vm = Vm::default();

        vm.s.reg.pc = scenario.initial.pc;
        vm.s.reg.s = scenario.initial.s;
        vm.s.reg.a = scenario.initial.a;
        vm.s.reg.x = scenario.initial.x;
        vm.s.reg.y = scenario.initial.y;
        vm.s.reg.p = scenario.initial.p;
        for address_value in &scenario.initial.ram {
            vm.s.memory[address_value.address] = address_value.value;
        }

        fail_if_not_eq!(true, vm.step());

        fail_if_not_eq!(scenario.r#final.pc, vm.s.reg.pc);
        fail_if_not_eq!(scenario.r#final.s, vm.s.reg.s);
        fail_if_not_eq!(scenario.r#final.a, vm.s.reg.a);
        fail_if_not_eq!(scenario.r#final.x, vm.s.reg.x);
        fail_if_not_eq!(scenario.r#final.y, vm.s.reg.y);
        fail_if_not_eq!(scenario.r#final.p, vm.s.reg.p);
        for address_value in &scenario.r#final.ram {
            fail_if_not_eq!(address_value.value, vm.s.memory[address_value.address]);
        }

        true
    }
}
