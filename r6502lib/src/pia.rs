use crate::MemoryMappedDevice;
use getch_rs::{Getch, Key};
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
enum Message {
    KbdUpdated,
    KbdcrUpdated,
    DspUpdated,
    DspcrUpdated,
    Key(Key),
}

struct PiaState {
    started: bool,
    kbd: u8,
    kbdcr: u8,
    dsp: u8,
    dspcr: u8,
}

impl PiaState {
    fn new() -> Self {
        Self {
            started: false,
            kbd: 0,
            kbdcr: 0,
            dsp: 0,
            dspcr: 0,
        }
    }

    fn set_key(&mut self, c: char) {
        let c = c.to_ascii_uppercase();
        if c as u8 == 0 {
            todo!();
        }
        self.kbd = (c as u8) | 0x80;
        self.kbdcr |= 0x80;
    }
}

pub(crate) struct Pia {
    tx: Sender<Message>,
    state: Arc<Mutex<PiaState>>,
    _stdin_handle: JoinHandle<()>,
    _event_handle: JoinHandle<()>,
}

impl Pia {
    pub(crate) const START_ADDR: u16 = 0xd010;
    pub(crate) const END_ADDR: u16 = 0xd013;

    // Apple I PIA addresses etc.
    #[allow(unused)]
    pub(crate) const KBD: u16 = 0xD010; // PIA.A keyboard input
    #[allow(unused)]
    pub(crate) const KBDCR: u16 = 0xD011; // PIA.A keyboard control register
    #[allow(unused)]
    pub(crate) const DSP: u16 = 0xD012; // PIA.B display output register
    #[allow(unused)]
    pub(crate) const DSPCR: u16 = 0xD013; //  PIA.B display control register
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
                        Key::Char(c) => {
                            let mut state = state_clone.lock().expect("Must succeed");
                            state.set_key(c)
                        }
                        Key::Delete => {
                            let mut state = state_clone.lock().expect("Must succeed");
                            state.set_key('_')
                        }
                        Key::Esc => {
                            let mut state = state_clone.lock().expect("Must succeed");
                            state.set_key(0x1b as char)
                        }
                        Key::Ctrl('c') => break,
                        _ => todo!(),
                    },
                    Message::KbdcrUpdated => {
                        let mut state = state_clone.lock().expect("Must succeed");
                        state.kbdcr = 0x00
                    }
                    Message::DspUpdated => {
                        let value = {
                            let mut state = state_clone.lock().expect("Must succeed");
                            let value = state.dsp;
                            state.dsp = 0x00;
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
        let mut state = self.state.lock().expect("Must succeed");
        state.started = true
    }

    fn load(&self, addr: u16) -> u8 {
        match addr {
            Self::KBD => {
                let mut state = self.state.lock().expect("Must succeed");
                let value = state.kbd;
                state.kbdcr = value & 0x7f;
                value
            }
            Self::KBDCR => {
                let state = self.state.lock().expect("Must succeed");
                state.kbdcr
            }
            Self::DSP => {
                let state = self.state.lock().expect("Must succeed");
                state.dsp
            }
            Self::DSPCR => {
                let state = self.state.lock().expect("Must succeed");
                state.dspcr
            }
            _ => panic!("Invalid PIA address ${addr:04X}"),
        }
    }

    fn store(&self, addr: u16, value: u8) {
        let started = {
            let state = self.state.lock().expect("Must succeed");
            state.started
        };
        if !started {
            return;
        }

        let m = match addr {
            Self::KBD => {
                let mut state = self.state.lock().expect("Must succeed");
                state.kbd = value;
                Message::KbdUpdated
            }
            Self::KBDCR => {
                let mut state = self.state.lock().expect("Must succeed");
                state.kbdcr = value;
                Message::KbdcrUpdated
            }
            Self::DSP => {
                let mut state = self.state.lock().expect("Must succeed");
                state.dsp = value;
                Message::DspUpdated
            }
            Self::DSPCR => {
                let mut state = self.state.lock().expect("Must succeed");
                state.dspcr = value;
                Message::DspcrUpdated
            }
            _ => panic!("Invalid PIA address ${addr:04X}"),
        };
        _ = self.tx.send(m)
    }
}
