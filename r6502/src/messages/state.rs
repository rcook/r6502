#[derive(Clone, Copy)]
pub enum State {
    Running,
    Stepping,
    Halted,
    Stopped,
}
