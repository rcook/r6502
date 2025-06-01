use crate::{Cycles, Instruction, MachineState, RegisterFile, Status};
use r6502lib::{InstructionInfo, Reg, TotalCycles};

pub(crate) struct PollResult {
    pub(crate) is_active: bool,
    pub(crate) free_running: bool,
}

pub(crate) trait VmHost {
    fn report_before_execute(
        &self,
        _total_cycles: TotalCycles,
        _reg: Reg,
        _instruction_info: InstructionInfo,
    ) {
    }

    fn report_after_execute(
        &self,
        _total_cycles: TotalCycles,
        _reg: Reg,
        _instruction_info: InstructionInfo,
    ) {
    }

    fn report_status(&self, _status: Status) {}

    fn write_stdout(&self, _c: char) {}
}
