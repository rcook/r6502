use crate::emulator::{BusDevice, BusEvent};
use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::cell::Cell;
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
enum Message {
    PaUpdated,
    PacrUpdated,
    PbUpdated,
    PbcrUpdated,
    Key(KeyEvent),
}

struct PiaState {
    started: bool,
    pa: u8,    // PIA.A keyboard input on Apple 1
    pa_cr: u8, // PIA.A keyboard control register on Apple 1
    pb: u8,    // PIA.B display output register on Apple 1
    pb_cr: u8, // PIA.B display control register on Apple 1
}

impl PiaState {
    const fn new() -> Self {
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

pub struct Pia {
    message_tx: Sender<Message>,
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
    #[must_use]
    pub fn new(bus_tx: Sender<BusEvent>) -> Self {
        let (message_tx, message_rx) = channel();
        let message_tx_clone = message_tx.clone();
        let stdin_handle =
            spawn(move || Self::stdin_loop(&message_tx_clone, &bus_tx).expect("Must succeed"));

        let state = Arc::new(Mutex::new(PiaState::new()));
        let state_clone = Arc::clone(&state);
        let event_handle =
            spawn(move || Self::event_loop(&message_rx, &state_clone).expect("Must succeed"));

        Self {
            message_tx,
            state,
            stdin_handle: Cell::new(Some(stdin_handle)),
            event_handle: Cell::new(Some(event_handle)),
        }
    }

    fn is_event_available() -> Result<bool> {
        Ok(poll(Duration::from_millis(100))?)
    }

    fn stdin_loop(message_tx: &Sender<Message>, bus_tx: &Sender<BusEvent>) -> Result<()> {
        enable_raw_mode()?;
        loop {
            if Self::is_event_available()? {
                let event = read()?;
                match event {
                    Event::Key(key) if key.is_press() => {
                        _ = message_tx.send(Message::Key(key));
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')
                        {
                            _ = bus_tx.send(BusEvent::UserBreak);
                            break;
                        }
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s')
                        {
                            _ = bus_tx.send(BusEvent::Snapshot);
                        }
                    }
                    _ => {}
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn event_loop(message_rx: &Receiver<Message>, state: &Arc<Mutex<PiaState>>) -> Result<()> {
        let mut stdout = stdout();
        loop {
            match message_rx.recv()? {
                Message::Key(key) => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Ok(()),
                    (KeyModifiers::CONTROL, KeyCode::Char('s')) => {}
                    (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                        state.lock().unwrap().set_key(c);
                    }
                    (KeyModifiers::NONE, KeyCode::Delete) => state.lock().unwrap().set_key('_'),
                    (KeyModifiers::NONE, KeyCode::Enter) => {
                        state.lock().unwrap().set_key(0x0d as char);
                    }
                    (KeyModifiers::NONE, KeyCode::Esc) => {
                        state.lock().unwrap().set_key(0x1b as char);
                    }
                    _ => todo!("{key:?}"),
                },
                Message::PacrUpdated => state.lock().unwrap().pa_cr = 0x00,
                Message::PbUpdated => {
                    let value = {
                        let mut state = state.lock().unwrap();
                        let value = state.pb;
                        state.pb = 0x00;
                        value
                    };
                    let char_value = value & 0x7f;
                    let ch = char_value as char;
                    match char_value {
                        0 => {}
                        13 => _ = stdout.write(&[13, 10]),
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
    }
}

impl BusDevice for Pia {
    fn start(&self) {
        self.state.lock().unwrap().started = true;
    }

    fn load(&self, addr: u16) -> u8 {
        let value = match addr {
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
        };

        value
    }

    fn store(&self, addr: u16, value: u8) {
        if !self.state.lock().unwrap().started {
            return;
        }

        let m = match addr {
            Self::PA_OFFSET => {
                self.state.lock().unwrap().pa = value;
                Message::PaUpdated
            }
            Self::PA_CR_OFFSET => {
                self.state.lock().unwrap().pa_cr = value;
                Message::PacrUpdated
            }
            Self::PB_OFFSET => {
                self.state.lock().unwrap().pb = value;
                Message::PbUpdated
            }
            Self::PB_CR_OFFSET => {
                self.state.lock().unwrap().pb_cr = value;
                Message::PbcrUpdated
            }
            _ => panic!("Invalid PIA address ${addr:04X}"),
        };
        _ = self.message_tx.send(m);
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
