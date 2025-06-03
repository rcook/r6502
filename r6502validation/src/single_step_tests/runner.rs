use crate::single_step_tests::{AddressValue, Scenario, ScenarioConfig, State};
use anyhow::{anyhow, bail, Result};
use r6502lib::{Opcode, Vm};
use std::ffi::OsStr;
use std::fs::{remove_file, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::panic::catch_unwind;
use std::path::Path;
use std::sync::LazyLock;

static LOG_PATH: LazyLock<&Path> = LazyLock::new(|| Path::new("failures.log"));

pub(crate) fn run_scenarios_with_filter(filter: &Option<String>) -> Result<()> {
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
                let result =
                    catch_unwind(|| run_scenario_inner(&scenario, false)).unwrap_or_default();
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

pub(crate) fn run_scenario_from_json(json: &str) -> Result<()> {
    let scenario = serde_json::from_str::<Scenario>(json)?;
    println!("{scenario}");
    if run_scenario_inner(&scenario, true) {
        println!("Scenario passed")
    } else {
        println!("Scenario failed")
    }
    Ok(())
}

fn run_scenario_inner(scenario: &Scenario, show_final_state: bool) -> bool {
    macro_rules! fail_if_not_eq {
        ($expected: expr, $actual: expr) => {
            let expected = $expected;
            let actual = $actual;
            if actual != expected {
                println!(
                    "Scenario \"{}\": assert failed at {}:{} (actual({actual:?}) != expected({expected:?}))",
                    scenario.name,
                    file!(),
                    line!()
                );
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

    if show_final_state {
        let mut ram = Vec::new();
        for address_value in &scenario.r#final.ram {
            ram.push(AddressValue {
                address: address_value.address,
                value: vm.s.memory[address_value.address],
            });
        }
        let state = State {
            pc: vm.s.reg.pc,
            s: vm.s.reg.s,
            a: vm.s.reg.a,
            x: vm.s.reg.x,
            y: vm.s.reg.y,
            p: vm.s.reg.p,
            ram,
        };
        println!("Actual:\n{state}");
    }

    fail_if_not_eq!(scenario.r#final.pc, vm.s.reg.pc);
    fail_if_not_eq!(scenario.r#final.s, vm.s.reg.s);
    fail_if_not_eq!(scenario.r#final.a, vm.s.reg.a);
    fail_if_not_eq!(scenario.r#final.x, vm.s.reg.x);
    fail_if_not_eq!(scenario.r#final.y, vm.s.reg.y);
    fail_if_not_eq!(scenario.r#final.p.bits(), vm.s.reg.p.bits());
    for address_value in &scenario.r#final.ram {
        fail_if_not_eq!(address_value.value, vm.s.memory[address_value.address]);
    }

    true
}

fn init_messages() -> Result<()> {
    match remove_file(*LOG_PATH) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(e) => bail!(e),
    }
    _ = File::create_new(*LOG_PATH)?;
    Ok(())
}

fn record_message(s: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(*LOG_PATH)?;
    writeln!(file, "{s}")?;
    Ok(())
}
