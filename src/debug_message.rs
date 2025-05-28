pub(crate) enum DebugMessage {
    Step,
    Run,
    Break,
    FetchMemory { begin: u16, end: u16 },
}
