use r6502lib::emulator::AddressRange;

pub(crate) enum DebugMessage {
    Step,
    Run,
    Break,
    FetchMemory(AddressRange),
    SetPc(u16),
    Go(u16),
}
