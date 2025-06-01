use crate::{Cycles, InstructionInfo, Reg, P_STR};

pub trait Monitor {
    fn on_before_fetch(&self, _reg: &Reg) {}
    fn on_before_execute(&self, _reg: &Reg, _instruction_info: &InstructionInfo) {}
    fn on_after_execute(&self, _reg: &Reg, _instruction_info: &InstructionInfo, _cycles: Cycles) {}
}

pub struct DummyMonitor;

impl Monitor for DummyMonitor {}

pub struct TracingMonitor;

impl Monitor for TracingMonitor {
    fn on_before_execute(&self, reg: &Reg, instruction_info: &InstructionInfo) {
        println!(
            "A({:02X}) X({:02X}) Y({:02X}) {}({:08b})  {}",
            reg.a,
            reg.x,
            reg.y,
            P_STR,
            reg.p.bits(),
            instruction_info.disassembly().expect("Must succeed")
        )
    }
}
