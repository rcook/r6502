use crate::MemoryMappedDevice;
use getch_rs::{Getch, Key};
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
            pa: 0,
            pa_cr: 0,
            pb: 0,
            pb_cr: 0,
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
    _stdin_handle: JoinHandle<()>,
    _event_handle: JoinHandle<()>,
}

impl Pia {
    const PA_OFFSET: u16 = 0x0000;
    const PA_CR_OFFSET: u16 = 0x0001;
    const PB_OFFSET: u16 = 0x0002;
    const PB_CR_OFFSET: u16 = 0x0003;
}

impl Default for Pia {
    fn default() -> Self {
        Self::new()
    }
}

impl Pia {
    fn new() -> Self {
        let (tx, rx) = channel();
        let state = Arc::new(Mutex::new(PiaState::new()));

        let tx_clone = tx.clone();
        let stdin_handle = spawn(move || {
            let g = Getch::new();
            loop {
                let key = g.getch().expect("Must succeed");
                //info!("keypress: {key:?}");
                _ = tx_clone.send(Message::Key(key.clone()));
                if matches!(key, Key::Ctrl('c')) {
                    break;
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
                                    _ = stdout.write(&[char_value])
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
            _stdin_handle: stdin_handle,
            _event_handle: event_handle,
        }
    }
}

impl MemoryMappedDevice for Pia {
    fn start(&self) {
        self.state.lock().expect("Must succeed").started = true
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

        // if log_enabled!(Level::Info) {
        //     let name = match addr {
        //         Self::PA_OFFSET => "PA (KBD)",
        //         Self::PA_CR_OFFSET => "PA_CR (KBDCR)",
        //         Self::PB_OFFSET => "PB (DSP)",
        //         Self::PB_CR_OFFSET => "PB_CR (DSPCR)",
        //         _ => panic!("Invalid PIA address ${addr:04X}"),
        //     };
        //     info!("PIA load: addr=${addr:04X} value=${value:02X} (0b{value:08b}) [({name})]");
        // }

        value
    }

    fn store(&self, addr: u16, value: u8) {
        // if log_enabled!(Level::Info) {
        //     let name = match addr {
        //         Self::PA_OFFSET => "PA (KBD)",
        //         Self::PA_CR_OFFSET => "PA_CR (KBDCR)",
        //         Self::PB_OFFSET => "PB (DSP)",
        //         Self::PB_CR_OFFSET => "PB_CR (DSPCR)",
        //         _ => panic!("Invalid PIA address ${addr:04X}"),
        //     };
        //     info!("PIA store: addr=${addr:04X} value=${value:02X} (0b{value:08b}) [({name})]");
        // }

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
        _ = self.tx.send(m)
    }
}
