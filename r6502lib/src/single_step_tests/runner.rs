use crate::single_step_tests::{Scenario, ScenarioConfig};
use crate::Vm;
use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::panic::catch_unwind;
use std::path::Path;

pub fn run_scenarios(filter: &Option<String>) -> Result<()> {
    let config = ScenarioConfig::new(filter)?;

    let mut count = 0;
    let mut failure_count = 0;

    for path in &config.paths {
        let scenarios = config.read_scenario_file(path)?;
        println!(
            "Running {} scenarios defined in {}",
            scenarios.len(),
            path.display()
        );

        for (file_name, scenario) in scenarios.iter() {
            let result = match catch_unwind(|| run_scenario(&scenario)) {
                Ok(result) => result,
                Err(_) => false,
            };
            if !result {
                println!(
                    "Scenario \"{}\" failed: rerun with --filter \"{},{}\"",
                    scenario.name, file_name, scenario.name
                );
                record_failure(scenario)?;
                failure_count += 1;
            }

            count += 1;
        }
    }

    if failure_count > 0 {
        panic!("Out of {count} scenarios, {failure_count} failed")
    } else {
        println!("All {count} scenarios passed")
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

fn record_failure(scenario: &Scenario) -> Result<()> {
    let path = Path::new("failures.log");
    let mut file = if path.is_file() {
        OpenOptions::new().write(true).append(true).open(path)?
    } else {
        File::create(path)?
    };
    writeln!(file, "{}", scenario.name)?;
    Ok(())
}
