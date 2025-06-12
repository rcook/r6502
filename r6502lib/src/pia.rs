use crate::{BusDevice, BusEvent};
use getch_rs::{Getch, Key};
use std::cell::Cell;
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
enum Message {
    PaUpdated,
    PacrUpdated,
    PbUpdated,
    PbcrUpdated,
    Key(Key),
}

struct PiaState {
    started: bool,
    pa: u8,    // PIA.A keyboard input on Apple 1
    pa_cr: u8, // PIA.A keyboard control register on Apple 1
    pb: u8,    // PIA.B display output register on Apple 1
    pb_cr: u8, // PIA.B display control register on Apple 1
}

impl PiaState {
    fn new() -> Self {
        Self {
            started: false,
            pa: 0x00,
            pa_cr: 0x00,
            pb: 0x00,
            pb_cr: 0x00,
        }
    }

    fn set_key(&mut self, c: char) {
        let c = c.to_ascii_uppercase();
        if c as u8 == 0 {
            todo!();
        }
        self.pa = (c as u8) | 0x80;
        self.pa_cr |= 0x80;
    }
}

pub(crate) struct Pia {
    tx: Sender<Message>,
    state: Arc<Mutex<PiaState>>,
    stdin_handle: Cell<Option<JoinHandle<()>>>,
    event_handle: Cell<Option<JoinHandle<()>>>,
}

impl Pia {
    const PA_OFFSET: u16 = 0x0000;
    const PA_CR_OFFSET: u16 = 0x0001;
    const PB_OFFSET: u16 = 0x0002;
    const PB_CR_OFFSET: u16 = 0x0003;
}

impl Pia {
    pub(crate) fn new(bus_tx: Sender<BusEvent>) -> Self {
        let (tx, rx) = channel();
        let state = Arc::new(Mutex::new(PiaState::new()));

        let tx_clone = tx.clone();
        let stdin_handle = spawn(move || {
            let g = Getch::new();
            loop {
                let key = g.getch().expect("Must succeed");
                _ = tx_clone.send(Message::Key(key.clone()));
                if matches!(key, Key::Ctrl('c')) {
                    _ = bus_tx.send(BusEvent::HardwareBreak);
                    break;
                }
                if matches!(key, Key::Ctrl('s')) {
                    _ = bus_tx.send(BusEvent::Snapshot);
                }
            }
        });

        let state_clone = Arc::clone(&state);
        let event_handle = spawn(move || {
            let mut stdout = stdout();
            loop {
                match rx.recv().expect("Must succeed") {
                    Message::Key(key) => match key {
                        Key::Char(c) => state_clone.lock().expect("Must succeed").set_key(c),
                        Key::Delete => state_clone.lock().expect("Must succeed").set_key('_'),
                        Key::Esc => state_clone
                            .lock()
                            .expect("Must succeed")
                            .set_key(0x1b as char),
                        Key::Ctrl('c') => break,
                        Key::Ctrl('s') => {}
                        _ => todo!(),
                    },
                    Message::PacrUpdated => state_clone.lock().expect("Must succeed").pa_cr = 0x00,
                    Message::PbUpdated => {
                        let value = {
                            let mut state = state_clone.lock().expect("Must succeed");
                            let value = state.pb;
                            state.pb = 0x00;
                            value
                        };
                        let char_value = value & 0x7f;
                        let ch = char_value as char;
                        match char_value {
                            0 => {}
                            13 => _ = stdout.write(&[10]),
                            _ => {
                                if !ch.is_control() {
                                    _ = stdout.write(&[char_value]);
                                }
                            }
                        }
                        if char_value != 0 {
                            _ = stdout.flush();
                        }
                    }
                    _ => {}
                }
            }
        });

        Self {
            tx,
            state,
            stdin_handle: Cell::new(Some(stdin_handle)),
            event_handle: Cell::new(Some(event_handle)),
        }
    }
}

impl BusDevice for Pia {
    fn start(&self) {
        self.state.lock().expect("Must succeed").started = true;
    }

    fn load(&self, addr: u16) -> u8 {
        let value = match addr {
            Self::PA_OFFSET => {
                let mut state = self.state.lock().expect("Must succeed");
                let value = state.pa;
                state.pa_cr = value & 0x7f;
                value
            }
            Self::PA_CR_OFFSET => self.state.lock().expect("Must succeed").pa_cr,
            Self::PB_OFFSET => self.state.lock().expect("Must succeed").pb,
            Self::PB_CR_OFFSET => self.state.lock().expect("Must succeed").pb_cr,
            _ => panic!("Invalid PIA address ${addr:04X}"),
        };

        value
    }

    fn store(&self, addr: u16, value: u8) {
        if !self.state.lock().expect("Must succeed").started {
            return;
        }

        let m = match addr {
            Self::PA_OFFSET => {
                self.state.lock().expect("Must succeed").pa = value;
                Message::PaUpdated
            }
            Self::PA_CR_OFFSET => {
                self.state.lock().expect("Must succeed").pa_cr = value;
                Message::PacrUpdated
            }
            Self::PB_OFFSET => {
                self.state.lock().expect("Must succeed").pb = value;
                Message::PbUpdated
            }
            Self::PB_CR_OFFSET => {
                self.state.lock().expect("Must succeed").pb_cr = value;
                Message::PbcrUpdated
            }
            _ => panic!("Invalid PIA address ${addr:04X}"),
        };
        _ = self.tx.send(m);
    }

    fn join(&self) {
        if let Some(h) = self.stdin_handle.take() {
            _ = h.join();
        }
        if let Some(h) = self.event_handle.take() {
            _ = h.join();
        }
    }
}
