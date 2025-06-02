#[cfg(test)]
mod tests {
    use crate::single_step_tests::{Scenario, ScenarioConfig, ENV_NAME};
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

    #[test]
    fn scenarios() -> Result<()> {
        let config = ScenarioConfig::from_env(&SKIPPED_SCENARIO_NAMES)?;

        let mut count = 0;
        let mut failure_count = 0;

        for path in &config.paths {
            let scenarios = config.read_scenario_file(path)?;
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
        }

        if failure_count > 0 {
            panic!("Out of {count} scenarios, {failure_count} failed")
        }

        Ok(())
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
