use crate::{Cycles, Instruction, MachineState, RegisterFile, Status};

pub(crate) struct PollResult {
    pub(crate) is_active: bool,
    pub(crate) free_running: bool,
}

pub(crate) trait VmHost {
    fn report_before_execute(
        &self,
        _reg: &RegisterFile,
        _cycles: Cycles,
        _instruction: &Instruction,
    ) {
    }

    fn report_after_execute(
        &self,
        _reg: &RegisterFile,
        _cycles: Cycles,
        _instruction: &Instruction,
    ) {
    }

    fn report_status(&self, _status: Status) {}

    fn write_stdout(&self, _c: char) {}
}
