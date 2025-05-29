use crate::{Cycles, MachineState};

#[derive(Debug, PartialEq)]
pub(crate) enum RunVMStatus {
    Halted,
    Disconnected,
}

#[allow(unused)]
pub(crate) struct RunVMResult {
    pub(crate) status: RunVMStatus,
    pub(crate) machine_state: MachineState,
    pub(crate) cycles: Cycles,
}

impl RunVMResult {
    pub(crate) fn new(status: RunVMStatus, machine_state: MachineState, cycles: Cycles) -> Self {
        Self {
            status,
            machine_state,
            cycles,
        }
    }
}
