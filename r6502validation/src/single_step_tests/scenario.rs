use crate::single_step_tests::{AddressValue, Cycle, State};
use anyhow::{anyhow, Result};
use r6502lib::Vm;
use serde::Deserialize;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    panic::catch_unwind,
};

#[derive(Debug, Deserialize)]
pub(crate) struct Scenario {
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
    pub(crate) fn from_json(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(|e| anyhow!(e))
    }

    pub(crate) fn run(&self) -> (bool, Option<State>) {
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
}

impl Display for Scenario {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Scenario: {}", self.name)?;
        write!(f, "Initial:\n{}", self.initial)?;
        write!(f, "Final:\n{}", self.r#final)
    }
}
