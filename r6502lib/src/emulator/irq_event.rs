use crate::emulator::Channel;

pub enum IrqEvent {
    Irq,
}

pub type IrqChannel = Channel<IrqEvent>;
