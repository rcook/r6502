use crate::Reg;
use crate::emulator::InstructionInfo;
use r6502lib::TotalCycles;

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
