use crate::{Cycles, Instruction, MachineState, RegisterFile, Status};

pub(crate) struct PollResult {
    pub(crate) is_active: bool,
    pub(crate) free_running: bool,
}

pub(crate) trait VMHost {
    fn report_before_execute(
        &self,
        _reg: &RegisterFile,
        _cycles: Cycles,
        _instruction: &Instruction,
    ) {
    }

    fn poll(&self, _machine_state: &MachineState, free_running: bool) -> PollResult {
        PollResult {
            is_active: true,
            free_running,
        }
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
