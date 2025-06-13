use crate::emulator::{AddressRange, InstructionInfo, Reg, TotalCycles};
use crate::messages::State;

pub enum MonitorMessage {
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
