use anyhow::Result;
use log::info;
use r6502config::CharSet;
use r6502core::keyboard::{KeyCode, KeyEvent, KeyModifiers};
use r6502cpu::{BusDevice, InterruptEvent};
use r6502lib::emulator::IoEvent::{
    self, Input, PaUpdated, PacrUpdated, PbUpdated, PbcrUpdated, Shutdown,
};
use r6502lib::emulator::char_set_util::translate_in;
use r6502lib::emulator::{BusEvent, IoChannel, OutputDevice};
use std::cell::Cell;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};

struct InterfaceAdapterState {
    started: bool,
    pa: u8,    // Port A
    pa_cr: u8, // Port A control register
    pb: u8,    // Port B
    pb_cr: u8, // Port B control register
}

impl InterfaceAdapterState {
    const fn new() -> Self {
        Self {
            started: false,
            pa: 0x00,
            pa_cr: 0x00,
            pb: 0x00,
            pb_cr: 0x00,
        }
    }

    const fn set_key(&mut self, value: u8) {
        self.pa = value;
        self.pa_cr |= 0x80;
    }
}

// A barely adequate emulation of the 6821 PIA and 6522 VIA
pub struct InterfaceAdapter {
    state: Arc<Mutex<InterfaceAdapterState>>,
    io_tx: Sender<IoEvent>,
    handle: Cell<Option<JoinHandle<()>>>,
}

impl InterfaceAdapter {
    const PA_OFFSET: u16 = 0x0000;
    const PA_CR_OFFSET: u16 = 0x0001;
    const PB_OFFSET: u16 = 0x0002;
    const PB_CR_OFFSET: u16 = 0x0003;

    #[must_use]
    pub fn new(
        output: Box<dyn OutputDevice>,
        io_channel: IoChannel,
        bus_tx: Sender<BusEvent>,
        interrupt_tx: Sender<InterruptEvent>,
        char_set: CharSet,
    ) -> Self {
        let state = Arc::new(Mutex::new(InterfaceAdapterState::new()));
        let state_clone = Arc::clone(&state);
        let handle = spawn(move || {
            Self::event_loop(
                &state_clone,
                &io_channel.rx,
                &bus_tx,
                &interrupt_tx,
                output,
                char_set,
            )
            .expect("Must succeed");
        });
        Self {
            state,
            io_tx: io_channel.tx,
            handle: Cell::new(Some(handle)),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn event_loop(
        state: &Arc<Mutex<InterfaceAdapterState>>,
        io_rx: &Receiver<IoEvent>,
        bus_tx: &Sender<BusEvent>,
        interrupt_tx: &Sender<InterruptEvent>,
        mut output: Box<dyn OutputDevice>,
        char_set: CharSet,
    ) -> Result<()> {
        loop {
            match io_rx.recv() {
                Ok(PaUpdated(value)) => state.lock().unwrap().pa = value,
                Ok(PacrUpdated(_)) => state.lock().unwrap().pa_cr = 0x00,
                Ok(PbUpdated(value)) => {
                    output.write(&char_set, value)?;
                    state.lock().unwrap().pb = 0x00;
                }
                Ok(PbcrUpdated(value)) => state.lock().unwrap().pb_cr = value,
                Ok(Input(KeyEvent {
                    code: KeyCode::F(12),
                    modifiers: KeyModifiers::NONE,
                })) => _ = interrupt_tx.send(InterruptEvent::Reset),
                Ok(Input(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                })) => {
                    _ = bus_tx.send(BusEvent::UserBreak);
                    break;
                }
                Ok(Input(KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::CONTROL,
                })) => {
                    // Reset CPU: i.e. call the RESET vector etc.
                    _ = bus_tx.send(BusEvent::Reset);
                }
                Ok(Input(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                })) => {
                    // Save snapshot of memory to disc
                    _ = bus_tx.send(BusEvent::Snapshot);
                }
                Ok(Input(key_event @ KeyEvent { .. })) => {
                    if let Some(c) = translate_in(&char_set, &key_event) {
                        state.lock().unwrap().set_key(c);
                        _ = interrupt_tx.send(InterruptEvent::Irq);
                    } else {
                        info!("unimplemented: {key_event:?}");
                    }
                }
                Ok(Shutdown) | Err(_) => break,
            }
        }

        Ok(())
    }
}

impl BusDevice for InterfaceAdapter {
    fn start(&self) {
        self.state.lock().unwrap().started = true;
    }

    fn stop(&self) -> bool {
        _ = self.io_tx.send(Shutdown);
        if let Some(h) = self.handle.take() {
            h.join().is_ok()
        } else {
            true
        }
    }

    fn load(&self, addr: u16) -> u8 {
        match addr {
            Self::PA_OFFSET => {
                let mut state = self.state.lock().unwrap();
                let value = state.pa;
                state.pa_cr = value & 0x7f;
                value
            }
            Self::PA_CR_OFFSET => self.state.lock().unwrap().pa_cr,
            Self::PB_OFFSET => self.state.lock().unwrap().pb,
            Self::PB_CR_OFFSET => self.state.lock().unwrap().pb_cr,
            _ => panic!("Invalid PIA address ${addr:04X}"),
        }
    }

    fn store(&self, addr: u16, value: u8) {
        if !self.state.lock().unwrap().started {
            return;
        }

        let m = match addr {
            Self::PA_OFFSET => PaUpdated(value),
            Self::PA_CR_OFFSET => PacrUpdated(value),
            Self::PB_OFFSET => PbUpdated(value),
            Self::PB_CR_OFFSET => PbcrUpdated(value),
            _ => panic!("Invalid PIA address ${addr:04X}"),
        };
        _ = self.io_tx.send(m);
    }
}
