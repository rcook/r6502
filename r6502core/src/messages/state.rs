#[derive(Clone, Copy, Debug)]
pub enum State {
    Running,
    Stepping,
    Halted,
    Stopped,
}
