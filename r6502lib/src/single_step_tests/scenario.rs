use crate::single_step_tests::{AddressValue, Cycle, ScenarioConfig, State};
use crate::{Opcode, Vm};
use anyhow::{anyhow, bail, Result};
use serde::Deserialize;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::{remove_file, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::panic::catch_unwind;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Scenario {
    #[serde(rename = "name")]
    pub(crate) name: String,

    #[serde(rename = "initial")]
    pub(crate) initial: State,

    #[serde(rename = "final")]
    pub(crate) r#final: State,

    #[allow(unused)]
    #[serde(rename = "cycles")]
    pub(crate) cycles: Vec<Cycle>,
}

impl Scenario {
    pub fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(|e| anyhow!(e))
    }

    pub fn run_scenarios_with_filter(report_path: &Path, filter: &Option<String>) -> Result<()> {
        let config = ScenarioConfig::new(filter)?;

        let mut all_total_count = 0;
        let mut all_failure_count = 0;
        let mut skipped_opcode_count = 0;

        let mut failure_counts = Vec::new();

        Self::init_messages(report_path)?;

        for path in &config.paths {
            let opcode_value = u8::from_str_radix(
                path.file_stem()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| anyhow!("Invalid path {}", path.display()))?,
                16,
            )?;
            match Opcode::from_u8(opcode_value) {
                None => {
                    Self::record_message(
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
                        let (result, _) = scenario.run();
                        if !result {
                            println!(
                                "Scenario \"{}\" failed: rerun with --filter \"{}\"",
                                scenario.name, scenario.name
                            );
                            Self::record_message(report_path, &scenario.name)?;
                            failure_count += 1;
                        }
                        total_count += 1;
                    }

                    failure_counts.push((opcode, failure_count));

                    all_total_count += total_count;
                    all_failure_count += failure_count;
                }
            }
        }

        if !failure_counts.is_empty() {
            failure_counts.sort_by(|a, b| b.1.cmp(&a.1));
            Self::record_message(report_path, "Failure counts:")?;
            for p in failure_counts {
                Self::record_message(report_path, &format!("{} {}", p.0, p.1))?;
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

    pub fn run(&self) -> (bool, Option<State>) {
        match catch_unwind(|| self.run_inner()) {
            Ok((result, state)) => (result, Some(state)),
            Err(_) => (false, None),
        }
    }

    fn run_inner(&self) -> (bool, State) {
        let mut vm = Vm::default();
        vm.s.reg.pc = self.initial.pc;
        vm.s.reg.s = self.initial.s;
        vm.s.reg.a = self.initial.a;
        vm.s.reg.x = self.initial.x;
        vm.s.reg.y = self.initial.y;
        vm.s.reg.p = self.initial.p;
        for address_value in &self.initial.ram {
            vm.s.memory[address_value.address] = address_value.value;
        }

        let final_state = State {
            pc: vm.s.reg.pc,
            s: vm.s.reg.s,
            a: vm.s.reg.a,
            x: vm.s.reg.x,
            y: vm.s.reg.y,
            p: vm.s.reg.p,
            ram: self
                .r#final
                .ram
                .iter()
                .map(|address_value| AddressValue {
                    address: address_value.address,
                    value: vm.s.memory[address_value.address],
                })
                .collect(),
        };

        macro_rules! check {
            ($reg: ident) => {
                let expected = self.r#final.$reg;
                let actual = vm.s.reg.$reg;
                if actual != expected {
                    println!(
                        "Scenario \"{name}\": actual value ${actual:02X} ({actual}) for register {reg} does not match expected value ${expected:02X} ({expected}) ({file}:{line})",
                        name = self.name,
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
                        name = self.name,
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

        _ = vm.step();

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
        check!(s);
        check!(a);
        check!(x);
        check!(y);
        check!(p);

        for p in &self.r#final.ram {
            check!(p.address, p.value, vm.s.memory[p.address]);
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
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Scenario: {}", self.name)?;
        write!(f, "Initial:\n{}", self.initial)?;
        write!(f, "Final:\n{}", self.r#final)
    }
}
