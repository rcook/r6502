use crate::AddressRange;

pub(crate) enum DebugMessage {
    Step,
    Run,
    Break,
    FetchMemory(AddressRange),
}
