use crate::MemoryMappedDevice;
use getch_rs::{Getch, Key};
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
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
    started: AtomicBool,
    kbd: AtomicU8,
    kbdcr: AtomicU8,
    dsp: AtomicU8,
    dspcr: AtomicU8,
}

impl PiaState {
    fn new() -> Self {
        Self {
            started: AtomicBool::new(false),
            kbd: AtomicU8::new(0),
            kbdcr: AtomicU8::new(0),
            dsp: AtomicU8::new(0),
            dspcr: AtomicU8::new(0),
        }
    }

    fn set_key(&self, c: char) {
        let c = c.to_ascii_uppercase();
        if c as u8 == 0 {
            todo!();
        }
        self.kbd.store((c as u8) | 0x80, Ordering::SeqCst);
        _ = self
            .kbdcr
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
                Some(value | 0x80)
            });
    }
}

pub(crate) struct Pia {
    tx: Sender<Message>,
    state: Arc<PiaState>,
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
        let state = Arc::new(PiaState::new());

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
                        Key::Char(c) => state_clone.set_key(c),
                        Key::Delete => state_clone.set_key('_'),
                        Key::Esc => state_clone.set_key(0x1b as char),
                        Key::Ctrl('c') => break,
                        _ => todo!(),
                    },
                    Message::KbdcrUpdated => state_clone.kbdcr.store(0x00, Ordering::SeqCst),
                    Message::DspUpdated => {
                        let value = state_clone.dsp.swap(0x00, Ordering::SeqCst);
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
        self.state.started.store(true, Ordering::SeqCst)
    }

    fn load(&self, addr: u16) -> u8 {
        match addr {
            Self::KBD => {
                let value = self.state.kbd.load(Ordering::SeqCst);
                _ = self
                    .state
                    .kbdcr
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
                        Some(value & 0x7f)
                    })
                    .expect("Must succeed");
                value
            }
            Self::KBDCR => self.state.kbdcr.load(Ordering::SeqCst),
            Self::DSP => self.state.dsp.load(Ordering::SeqCst),
            Self::DSPCR => self.state.dspcr.load(Ordering::SeqCst),
            _ => panic!("Invalid PIA address ${addr:04X}"),
        }
    }

    fn store(&self, addr: u16, value: u8) {
        if self.state.started.load(Ordering::SeqCst) {
            let m = match addr {
                Self::KBD => {
                    self.state.kbd.store(value, Ordering::SeqCst);
                    Message::KbdUpdated
                }
                Self::KBDCR => {
                    self.state.kbdcr.store(value, Ordering::SeqCst);
                    Message::KbdcrUpdated
                }
                Self::DSP => {
                    self.state.dsp.store(value, Ordering::SeqCst);
                    Message::DspUpdated
                }
                Self::DSPCR => {
                    self.state.dspcr.store(value, Ordering::SeqCst);
                    Message::DspcrUpdated
                }
                _ => panic!("Invalid PIA address ${addr:04X}"),
            };
            _ = self.tx.send(m)
        } else {
            // Ignore: device has not started yet
        }
    }
}
