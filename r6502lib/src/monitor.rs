use crate::{Cycles, InstructionInfo, Reg};

pub(crate) trait Monitor {
    fn on_before_fetch(&self, _reg: &Reg) {}
    fn on_before_execute(&self, _reg: &Reg, _instruction: &InstructionInfo) {}
    fn on_after_execute(&self, _reg: &Reg, _instruction: &InstructionInfo, _cycles: Cycles) {}
}

#[allow(unused)]
pub(crate) struct DummyMonitor;

impl Monitor for DummyMonitor {}
