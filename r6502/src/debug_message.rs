use r6502lib::AddressRange;

pub(crate) enum DebugMessage {
    Step,
    Run,
    Break,
    FetchMemory(AddressRange),
    SetPc(u16),
    Go(u16),
}
