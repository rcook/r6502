use crate::{Cycles, Instruction, RegisterFile, Status};

pub(crate) struct PollResult {
    pub(crate) is_active: bool,
    pub(crate) free_running: bool,
}

pub(crate) trait VMHost {
    fn report_before_execute(&self, reg: &RegisterFile, cycles: Cycles, instruction: &Instruction);
    fn poll(&self, free_running: bool) -> PollResult;
    fn report_after_execute(&self, reg: &RegisterFile, cycles: Cycles, instruction: &Instruction);
    fn report_status(&self, status: Status);
    fn write_stdout(&self, c: char);
}
