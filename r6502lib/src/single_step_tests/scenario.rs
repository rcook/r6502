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

        let mut count = 0;
        let mut failure_count = 0;
        let mut skipped_opcode_count = 0;

        Self::init_messages(report_path)?;

        for path in &config.paths {
            let opcode_value = u8::from_str_radix(
                path.file_stem()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| anyhow!("Invalid path {}", path.display()))?,
                16,
            )?;
            if Opcode::from_u8(opcode_value).is_none() {
                Self::record_message(
                    report_path,
                    &format!("Unsupported opcode ${:02X}", opcode_value),
                )?;
                skipped_opcode_count += 1;
            } else {
                let scenarios = config.read_scenarios(path)?;
                println!(
                    "Running {} scenarios defined in {}",
                    scenarios.len(),
                    path.display()
                );

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

        let result = vm.step();

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

        macro_rules! fail_if_not_eq {
            ($expected: expr, $actual: expr) => {
                let expected = $expected;
                let actual = $actual;
                if actual != expected {
                    println!(
                        "Scenario \"{}\": assert failed at {}:{} (actual({actual:?}) != expected({expected:?}))",
                        self.name,
                        file!(),
                        line!()
                    );
                    return (false, final_state);
                }
            };
        }

        fail_if_not_eq!(true, result);

        fail_if_not_eq!(self.r#final.pc, vm.s.reg.pc);
        fail_if_not_eq!(self.r#final.s, vm.s.reg.s);
        fail_if_not_eq!(self.r#final.a, vm.s.reg.a);
        fail_if_not_eq!(self.r#final.x, vm.s.reg.x);
        fail_if_not_eq!(self.r#final.y, vm.s.reg.y);
        fail_if_not_eq!(self.r#final.p.bits(), vm.s.reg.p.bits());
        for address_value in &self.r#final.ram {
            fail_if_not_eq!(address_value.value, vm.s.memory[address_value.address]);
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
