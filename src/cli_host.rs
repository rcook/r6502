use crate::{Cycles, Instruction, PollResult, RegisterFile, Status, VMHost};

pub(crate) struct CliHost {}

impl CliHost {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl VMHost for CliHost {
    fn report_before_execute(
        &self,
        _reg: &RegisterFile,
        _cycles: Cycles,
        _instruction: &Instruction,
    ) {
    }

    fn poll(&self, free_running: bool) -> PollResult {
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

    fn write_stdout(&self, c: char) {
        print!("{c}")
    }
}
