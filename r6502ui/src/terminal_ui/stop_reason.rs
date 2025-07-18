use r6502cpu::TotalCycles;

pub enum StopReason {
    UnexpectedInterrupt { total_cycles: TotalCycles },
    UserBreak { total_cycles: TotalCycles },
    RequestedCyclesExecuted { total_cycles: TotalCycles },
    Halt { total_cycles: TotalCycles, a: u8 },
}
