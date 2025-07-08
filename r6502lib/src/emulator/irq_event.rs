use crate::emulator::Channel;

pub enum IrqEvent {
    Notify,
}

pub type IrqChannel = Channel<IrqEvent>;
