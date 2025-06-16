use crate::emulator::{BusDevice, BusEvent, OutputDevice};
use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::cell::Cell;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

#[derive(Debug)]
enum StdinMessage {
    ShutDown,
}

#[derive(Debug)]
enum EventMessage {
    PaUpdated,
    PacrUpdated,
    PbUpdated,
    PbcrUpdated,
    ShutDown,
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
    stdin_tx: Sender<StdinMessage>,
    event_tx: Sender<EventMessage>,
    state: Arc<Mutex<PiaState>>,
    input_thread: Cell<Option<JoinHandle<()>>>,
    event_thread: Cell<Option<JoinHandle<()>>>,
}

impl Pia {
    const PA_OFFSET: u16 = 0x0000;
    const PA_CR_OFFSET: u16 = 0x0001;
    const PB_OFFSET: u16 = 0x0002;
    const PB_CR_OFFSET: u16 = 0x0003;

    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn new(
        output: Box<dyn OutputDevice>,
        event_rx: Receiver<Event>,
        bus_tx: Sender<BusEvent>,
    ) -> Self {
        let state = Arc::new(Mutex::new(PiaState::new()));
        let (stdin_tx, stdin_rx) = channel();
        let (event_tx2, event_rx2) = channel();

        // TBD: Fuse these two threads into a single event loop if possible

        let state_clone = Arc::clone(&state);
        let stdin_thread =
            spawn(move || Self::stdin_loop(&state_clone, event_rx, &stdin_rx, &bus_tx));

        let state_clone = Arc::clone(&state);
        let event_thread = spawn(move || {
            Self::event_loop(&state_clone, &event_rx2, output).expect("Must succeed");
        });

        Self {
            stdin_tx,
            event_tx: event_tx2,
            state,
            input_thread: Cell::new(Some(stdin_thread)),
            event_thread: Cell::new(Some(event_thread)),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn stdin_loop(
        state: &Arc<Mutex<PiaState>>,
        event_rx: Receiver<Event>,
        input_rx: &Receiver<StdinMessage>,
        bus_tx: &Sender<BusEvent>,
    ) {
        loop {
            match input_rx.try_recv() {
                Ok(StdinMessage::ShutDown) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            match event_rx.try_recv() {
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
                Ok(event) => match event {
                    Event::Key(key) if Self::key_event_is_press(&key) => {
                        match (key.modifiers, key.code) {
                            (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                                // Halt program
                                _ = bus_tx.send(BusEvent::UserBreak);
                                break;
                            }
                            (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                                // Reset CPU: i.e. call the RESET vector etc.
                                _ = bus_tx.send(BusEvent::Reset);
                            }
                            (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                                // Save snapshot of memory to disc
                                _ = bus_tx.send(BusEvent::Snapshot);
                            }
                            (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                                state.lock().unwrap().set_key(c);
                            }
                            (KeyModifiers::NONE, KeyCode::Backspace | KeyCode::Delete) => {
                                state.lock().unwrap().set_key('_');
                            }
                            (KeyModifiers::NONE, KeyCode::Enter) => {
                                state.lock().unwrap().set_key(0x0d as char);
                            }
                            (KeyModifiers::NONE, KeyCode::Esc) => {
                                state.lock().unwrap().set_key(0x1b as char);
                            }
                            _ => todo!("{key:?}"),
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn event_loop(
        state: &Arc<Mutex<PiaState>>,
        event_rx: &Receiver<EventMessage>,
        output: Box<dyn OutputDevice>,
    ) -> Result<()> {
        loop {
            match event_rx.recv()? {
                EventMessage::PaUpdated | EventMessage::PbcrUpdated => {}
                EventMessage::PacrUpdated => state.lock().unwrap().pa_cr = 0x00,
                EventMessage::PbUpdated => {
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
                        13 => output.write('\n')?,
                        _ => {
                            if !ch.is_control() {
                                output.write(ch)?;
                            }
                        }
                    }
                }
                EventMessage::ShutDown => break,
            }
        }

        Ok(())
    }

    // TBD: Use KeyEvent::is_press in crossterm >= 0.29
    const fn key_event_is_press(key: &KeyEvent) -> bool {
        matches!(key.kind, KeyEventKind::Press)
    }
}

impl BusDevice for Pia {
    fn start(&self) {
        self.state.lock().unwrap().started = true;
    }

    fn stop(&self) {
        _ = self.stdin_tx.send(StdinMessage::ShutDown);
        _ = self.event_tx.send(EventMessage::ShutDown);

        if let Some(h) = self.input_thread.take() {
            _ = h.join();
        }
        if let Some(h) = self.event_thread.take() {
            _ = h.join();
        }
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
                EventMessage::PaUpdated
            }
            Self::PA_CR_OFFSET => {
                self.state.lock().unwrap().pa_cr = value;
                EventMessage::PacrUpdated
            }
            Self::PB_OFFSET => {
                self.state.lock().unwrap().pb = value;
                EventMessage::PbUpdated
            }
            Self::PB_CR_OFFSET => {
                self.state.lock().unwrap().pb_cr = value;
                EventMessage::PbcrUpdated
            }
            _ => panic!("Invalid PIA address ${addr:04X}"),
        };
        _ = self.event_tx.send(m);
    }
}
