use r6502lib::TotalCycles;

pub enum StopReason {
    UnexpectedInterrupt { total_cycles: TotalCycles },
    UserBreak { total_cycles: TotalCycles },
    RequestedCyclesExecuted { total_cycles: TotalCycles },
    Halt { total_cycles: TotalCycles, a: u8 },
}
