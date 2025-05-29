use crate::{Cpu, Cycles, InstructionInfo, Reg, P_STR};

pub(crate) trait Monitor {
    fn on_before_fetch(&self, _reg: &Reg) {}
    fn on_before_execute(&self, _cpu: &Cpu, _reg: &Reg, _instruction: &InstructionInfo) {}
    fn on_after_execute(
        &self,
        _cpu: &Cpu,
        _reg: &Reg,
        _instruction: &InstructionInfo,
        _cycles: Cycles,
    ) {
    }
}

#[allow(unused)]
pub(crate) struct DummyMonitor;

impl Monitor for DummyMonitor {}

#[allow(unused)]
pub(crate) struct TracingMonitor;

impl Monitor for TracingMonitor {
    fn on_before_execute(&self, cpu: &Cpu, reg: &Reg, instruction: &InstructionInfo) {
        println!(
            "A({:02X}) X({:02X}) Y({:02X}) {}({:08b})  {}",
            reg.a,
            reg.x,
            reg.y,
            P_STR,
            reg.p.bits(),
            instruction.disassembly(cpu).expect("Must succeed")
        )
    }
}
