use crate::{Cpu, CpuState, DummyMonitor, Memory, Opcode, Reg, _p};
use anyhow::{anyhow, bail, Result};
use r6502validationlib::{AddressValue, Scenario, ScenarioConfig, State};
use std::ffi::OsStr;
use std::fs::{remove_file, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::panic::catch_unwind;
use std::path::Path;

pub fn run_scenarios_with_filter(report_path: &Path, filter: &Option<String>) -> Result<()> {
    let config = ScenarioConfig::new(filter)?;

    let mut all_total_count = 0;
    let mut all_failure_count = 0;
    let mut skipped_opcode_count = 0;

    let mut failure_counts = Vec::new();

    init_messages(report_path)?;

    for path in &config.paths {
        let opcode_value = u8::from_str_radix(
            path.file_stem()
                .and_then(OsStr::to_str)
                .ok_or_else(|| anyhow!("Invalid path {}", path.display()))?,
            16,
        )?;
        match Opcode::from_u8(opcode_value) {
            None => {
                record_message(
                    report_path,
                    &format!("Unsupported opcode ${:02X}", opcode_value),
                )?;
                skipped_opcode_count += 1;
            }
            Some(opcode) => {
                let scenarios = config.read_scenarios(path)?;
                println!(
                    "Running {} scenarios defined in {}",
                    scenarios.len(),
                    path.display()
                );

                let mut total_count = 0;
                let mut failure_count = 0;
                for scenario in scenarios {
                    let (result, _) = run_scenario(&scenario);
                    if !result {
                        println!(
                            "Scenario \"{}\" failed: rerun with --filter \"{}\"",
                            scenario.name, scenario.name
                        );
                        record_message(report_path, &scenario.name)?;
                        failure_count += 1;
                    }
                    total_count += 1;
                }

                if failure_count > 0 {
                    failure_counts.push((opcode, failure_count));
                }

                all_total_count += total_count;
                all_failure_count += failure_count;
            }
        }
    }

    if !failure_counts.is_empty() {
        failure_counts.sort_by(|a, b| b.1.cmp(&a.1));
        record_message(report_path, "Failure counts:")?;
        for p in failure_counts {
            record_message(report_path, &format!("{} {}", p.0, p.1))?;
        }
    }

    if all_failure_count > 0 {
        panic!("Out of {all_total_count} scenarios, {all_failure_count} failed")
    } else {
        println!("All {all_total_count} scenarios passed")
    }
    if skipped_opcode_count > 0 {
        println!("Skipped {skipped_opcode_count} unsupported opcodes");
    }

    Ok(())
}

pub fn run_scenario(scenario: &Scenario) -> (bool, Option<State>) {
    match catch_unwind(|| run_inner(scenario)) {
        Ok((result, state)) => (result, Some(state)),
        Err(_) => (false, None),
    }
}

fn run_inner(scenario: &Scenario) -> (bool, State) {
    let memory = Memory::default();
    let mut cpu = Cpu::new(
        Box::new(DummyMonitor),
        CpuState::new(Reg::default(), memory.view()),
    );
    cpu.s.reg.pc = scenario.initial.pc;
    cpu.s.reg.sp = scenario.initial.sp;
    cpu.s.reg.a = scenario.initial.a;
    cpu.s.reg.x = scenario.initial.x;
    cpu.s.reg.y = scenario.initial.y;
    cpu.s.reg.p = _p!(scenario.initial.p);
    for address_value in &scenario.initial.ram {
        cpu.s
            .memory
            .store(address_value.address, address_value.value);
    }

    _ = cpu.step();

    let final_state = State {
        pc: cpu.s.reg.pc,
        sp: cpu.s.reg.sp,
        a: cpu.s.reg.a,
        x: cpu.s.reg.x,
        y: cpu.s.reg.y,
        p: cpu.s.reg.p.bits(),
        ram: scenario
            .r#final
            .ram
            .iter()
            .map(|address_value| AddressValue {
                address: address_value.address,
                value: cpu.s.memory.load(address_value.address),
            })
            .collect(),
    };

    macro_rules! check {
            ($reg: ident) => {
                let expected = scenario.r#final.$reg;
                let actual = cpu.s.reg.$reg;
                if actual != expected {
                    println!(
                        "Scenario \"{name}\": actual value ${actual:02X} ({actual}) for register {reg} does not match expected value ${expected:02X} ({expected}) ({file}:{line})",
                        name = scenario.name,
                        file = file!(),
                        line = line!(),
                        reg = stringify!($reg),
                        expected = expected,
                        actual = actual,
                    );
                    return (false, final_state);
                }
            };
            ($addr: expr, $expected: expr, $actual: expr) => {
                let expected = $expected;
                let actual = $actual;
                if actual != expected {
                    println!(
                        "Scenario \"{name}\": actual value ${actual:02X} ({actual}) at location ${addr:04X} does not match expected value ${expected:02X} ({expected}) ({file}:{line})",
                        name = scenario.name,
                        file = file!(),
                        line = line!(),
                        addr = $addr,
                        expected = expected,
                        actual = actual,
                    );
                    return (false, final_state);
                }
            };
        }

    macro_rules! check_p {
        ($reg: ident) => {
            let expected = scenario.r#final.$reg;
            let actual = cpu.s.reg.$reg;
            if actual.bits() != expected {
                println!(
                    "Scenario \"{name}\": actual value ${actual:02X} ({actual}) for register {reg} does not match expected value ${expected:02X} ({expected}) ({file}:{line})",
                    name = scenario.name,
                    file = file!(),
                    line = line!(),
                    reg = stringify!($reg),
                    expected = expected,
                    actual = actual,
                );
                return (false, final_state);
            }
        };
    }

    /*
    if !result {
        panic!(
            "Scenario \"{name}\": step unexpectedly hit a breakpoint ({file}:{line})",
            name = self.name,
            file = file!(),
            line = line!(),
        )
    }
    */

    check!(pc);
    check!(sp);
    check!(a);
    check!(x);
    check!(y);
    check_p!(p);

    for p in &scenario.r#final.ram {
        check!(p.address, p.value, cpu.s.memory.load(p.address));
    }

    (true, final_state)
}

fn init_messages(report_path: &Path) -> Result<()> {
    match remove_file(report_path) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::NotFound => {}
        Err(e) => bail!(e),
    }
    _ = File::create_new(report_path)?;
    Ok(())
}

fn record_message(report_path: &Path, s: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(report_path)?;
    writeln!(file, "{s}")?;
    Ok(())
}
