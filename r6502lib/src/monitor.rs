use crate::{InstructionInfo, Reg, SymbolInfo, TotalCycles};

pub trait Monitor {
    fn on_before_execute(
        &self,
        _total_cycles: TotalCycles,
        _reg: Reg,
        _instruction_info: InstructionInfo,
    ) {
    }

    fn on_after_execute(
        &self,
        _total_cycles: TotalCycles,
        _reg: Reg,
        _instruction_info: InstructionInfo,
    ) {
    }
}

pub struct DummyMonitor;

impl Monitor for DummyMonitor {}

#[derive(Default)]
pub struct TracingMonitor {
    symbols: Vec<SymbolInfo>,
}

impl TracingMonitor {
    pub fn new(symbols: Vec<SymbolInfo>) -> Self {
        Self { symbols }
    }
}

impl Monitor for TracingMonitor {
    fn on_before_execute(
        &self,
        _total_cycles: TotalCycles,
        reg: Reg,
        instruction_info: InstructionInfo,
    ) {
        println!(
            "A({:02X}) X({:02X}) Y({:02X}) P({})  {}",
            reg.a,
            reg.x,
            reg.y,
            reg.p,
            instruction_info
                .disassembly(&self.symbols)
                .expect("Must succeed")
        )
    }
}
