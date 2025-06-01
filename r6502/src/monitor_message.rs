use crate::{AddressRange, Status};
use r6502lib::{InstructionInfo, Reg, TotalCycles};

pub(crate) enum MonitorMessage {
    BeforeFetch {
        #[allow(unused)]
        total_cycles: TotalCycles,
        #[allow(unused)]
        reg: Reg,
    },
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
    Status(Status),
    FetchMemoryResponse {
        address_range: AddressRange,
        snapshot: Vec<u8>,
    },
}
