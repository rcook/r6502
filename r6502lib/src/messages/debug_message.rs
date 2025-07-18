use r6502core::AddressRange;

pub enum DebugMessage {
    Step,
    Run,
    Break,
    FetchMemory(AddressRange),
    SetPc(u16),
    Go(u16),
}
