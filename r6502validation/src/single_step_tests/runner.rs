use crate::single_step_tests::{Scenario, ScenarioConfig};
use anyhow::{anyhow, bail, Result};
use r6502lib::{Opcode, Vm};
use std::ffi::OsStr;
use std::fs::{remove_file, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::panic::catch_unwind;
use std::path::Path;
use std::sync::LazyLock;

const LOG_PATH: LazyLock<&Path> = LazyLock::new(|| Path::new("failures.log"));

pub(crate) fn run_scenarios(filter: &Option<String>) -> Result<()> {
    let config = ScenarioConfig::new(filter)?;

    let mut count = 0;
    let mut failure_count = 0;
    let mut skipped_opcode_count = 0;

    init_messages()?;

    for path in &config.paths {
        let opcode_value = u8::from_str_radix(
            path.file_stem()
                .and_then(OsStr::to_str)
                .ok_or_else(|| anyhow!("Invalid path {}", path.display()))?,
            16,
        )?;
        if Opcode::from_u8(opcode_value).is_none() {
            record_message(&format!("Unsupported opcode ${:02X}", opcode_value))?;
            skipped_opcode_count += 1;
        } else {
            let scenarios = config.read_scenarios(path)?;
            println!(
                "Running {} scenarios defined in {}",
                scenarios.len(),
                path.display()
            );

            for scenario in scenarios {
                let result = match catch_unwind(|| run_scenario(&scenario)) {
                    Ok(result) => result,
                    Err(_) => false,
                };
                if !result {
                    println!(
                        "Scenario \"{}\" failed: rerun with --filter \"{}\"",
                        scenario.name, scenario.name
                    );
                    record_message(&scenario.name)?;
                    failure_count += 1;
                }

                count += 1;
            }
        }
    }

    if failure_count > 0 {
        panic!("Out of {count} scenarios, {failure_count} failed")
    } else {
        println!("All {count} scenarios passed")
    }
    if skipped_opcode_count > 0 {
        println!("Skipped {skipped_opcode_count} unsupported opcodes");
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

fn init_messages() -> Result<()> {
    match remove_file(&*LOG_PATH) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(e) => bail!(e),
    }
    _ = File::create_new(&*LOG_PATH)?;
    Ok(())
}

fn record_message(s: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&*LOG_PATH)?;
    writeln!(file, "{s}")?;
    Ok(())
}
