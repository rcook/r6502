use crate::{Cycles, Instruction, RegisterFile, Status};
use r6502lib::{InstructionInfo, Reg, TotalCycles};

pub(crate) enum MonitorMessage {
    BeforeFetch {
        total_cycles: TotalCycles,
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
        begin: u16,
        end: u16,
        snapshot: Vec<u8>,
    },
}
