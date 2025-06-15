use crate::emulator::{BusDevice, BusEvent, PiaBehaviour};
use anyhow::Result;
use cursive::backends::crossterm::crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
};
use std::cell::Cell;
use std::io::{stdout, Write};
use std::marker::PhantomData;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

enum InputMessage {
    ShutDown,
}

enum EventMessage {
    PaUpdated,
    PacrUpdated,
    PbUpdated,
    PbcrUpdated,
    Key(KeyEvent),
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

pub struct Pia<B: PiaBehaviour> {
    input_tx: Sender<InputMessage>,
    event_tx: Sender<EventMessage>,
    state: Arc<Mutex<PiaState>>,
    input: Cell<Option<JoinHandle<()>>>,
    event: Cell<Option<JoinHandle<()>>>,
    behaviour: PhantomData<B>,
}

impl<B: PiaBehaviour> Pia<B> {
    const PA_OFFSET: u16 = 0x0000;
    const PA_CR_OFFSET: u16 = 0x0001;
    const PB_OFFSET: u16 = 0x0002;
    const PB_CR_OFFSET: u16 = 0x0003;

    #[must_use]
    pub fn new(bus_tx: Sender<BusEvent>) -> Self {
        let (input_tx, input_rx) = channel();
        let (event_tx, event_rx) = channel();
        let temp = event_tx.clone();
        let input =
            spawn(move || Self::input_loop(&input_rx, &temp, &bus_tx).expect("Must succeed"));

        let state = Arc::new(Mutex::new(PiaState::new()));
        let temp = Arc::clone(&state);
        let event = spawn(move || Self::event_loop(&event_rx, &temp).expect("Must succeed"));

        Self {
            input_tx,
            event_tx,
            state,
            input: Cell::new(Some(input)),
            event: Cell::new(Some(event)),
            behaviour: PhantomData,
        }
    }

    fn input_loop(
        input_rx: &Receiver<InputMessage>,
        event_tx: &Sender<EventMessage>,
        bus_tx: &Sender<BusEvent>,
    ) -> Result<()> {
        B::enable_raw_mode();

        loop {
            match input_rx.try_recv() {
                Ok(InputMessage::ShutDown) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            if let Some(event) = B::try_read_event()? {
                match event {
                    Event::Key(key) if Self::key_event_is_press(&key) => {
                        _ = event_tx.send(EventMessage::Key(key));

                        // Halt program
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')
                        {
                            _ = bus_tx.send(BusEvent::UserBreak);
                            break;
                        }

                        // Reset
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('r')
                        {
                            _ = bus_tx.send(BusEvent::Reset);
                        }

                        // Save snapshot
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('s')
                        {
                            _ = bus_tx.send(BusEvent::Snapshot);
                        }
                    }
                    _ => {}
                }
            }
        }

        B::disable_raw_mode();

        Ok(())
    }

    fn event_loop(event_rx: &Receiver<EventMessage>, state: &Arc<Mutex<PiaState>>) -> Result<()> {
        let mut stdout = stdout();
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
                EventMessage::Key(key) => match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (KeyModifiers::CONTROL, KeyCode::Char('r' | 's')) => {}
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
                },
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

impl<B: PiaBehaviour> BusDevice for Pia<B> {
    fn start(&self) {
        self.state.lock().unwrap().started = true;
    }

    fn stop(&self) {
        _ = self.input_tx.send(InputMessage::ShutDown);
        _ = self.event_tx.send(EventMessage::ShutDown);

        if let Some(h) = self.input.take() {
            _ = h.join();
        }
        if let Some(h) = self.event.take() {
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
