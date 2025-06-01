#[derive(Clone, Copy)]
pub(crate) enum State {
    Running,
    Stepping,
    Halted,
    Stopped,
}
