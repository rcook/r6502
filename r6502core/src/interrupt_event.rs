use r6502lib::Channel;

pub enum InterruptEvent {
    Irq,
    #[allow(unused)]
    Nmi,
    #[allow(unused)]
    Reset,
}

pub type InterruptChannel = Channel<InterruptEvent>;
