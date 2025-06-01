use crate::{AddressRange, State};
use r6502lib::{InstructionInfo, Reg, TotalCycles};

pub(crate) enum MonitorMessage {
    NotifyState(State),
    NotifyInvalidBrk,
    BeforeExecute {
        total_cycles: TotalCycles,
        reg: Reg,
        instruction_info: InstructionInfo,
    },
    AfterExecute {
        total_cycles: TotalCycles,
        reg: Reg,
        instruction_info: InstructionInfo,
    },
    FetchMemoryResponse {
        address_range: AddressRange,
        snapshot: Vec<u8>,
    },
}
