use crate::constants::{QUIT_MESSAGE, QUIT_TITLE};
use crate::terminal::Terminal;
use crate::terminal_event::TerminalEvent;
use crate::tui::crossterm_key_util::translate_key_code;
use crate::util::Channel;
use anyhow::Result;
use crossterm::ExecutableCommand;
use crossterm::cursor::MoveTo;
use crossterm::event::{
    Event as Event_, KeyCode as KeyCode_, KeyEvent as KeyEvent_, KeyEventKind,
    KeyModifiers as KeyModifiers_, poll, read,
};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};
use log::{info, warn};
use r6502lib::keyboard::{KeyCode, KeyEvent, KeyModifiers};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use std::cell::RefCell;
use std::io::{Write, stdout};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::scope;
use std::time::Duration;

enum InternalEvent {
    Quit,
}

pub struct TextTerminal {
    internal_tx: Sender<InternalEvent>,
    column: RefCell<usize>,
}

impl TextTerminal {
    pub fn with<F, T>(action: F) -> Result<T>
    where
        F: FnOnce(&TextTerminal, &Receiver<TerminalEvent>) -> Result<T> + Send + Sync,
        T: Send,
    {
        struct RawMode;

        impl RawMode {
            fn new() -> Result<Self> {
                enable_raw_mode()?;
                Ok(Self)
            }
        }

        impl Drop for RawMode {
            fn drop(&mut self) {
                _ = disable_raw_mode();
            }
        }

        let internal_channel = Channel::<InternalEvent>::new();
        let me = Self {
            internal_tx: internal_channel.tx,
            column: RefCell::new(0),
        };

        info!("text terminal thread started");
        let result = scope(|scope| -> Result<T> {
            let raw_mode = RawMode::new()?;
            let terminal_channel = Channel::<TerminalEvent>::new();
            let handle = scope.spawn(move || action(&me, &terminal_channel.rx));

            loop {
                match internal_channel.rx.try_recv() {
                    Ok(InternalEvent::Quit) => {
                        if Self::confirm(QUIT_TITLE, QUIT_MESSAGE) {
                            break;
                        }
                    }
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => {}
                }

                if let Some(event) = Self::try_read_event()? {
                    if !Self::handle_event(&event, &terminal_channel.tx) {
                        break;
                    }
                }
            }

            _ = terminal_channel.tx.send(TerminalEvent::Closed);
            let result = handle.join().unwrap();
            drop(raw_mode);
            result
        })?;
        Ok(result)
    }

    fn confirm(title: &str, message: &str) -> bool {
        matches!(
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title(title)
                .set_description(message)
                .set_buttons(MessageButtons::YesNo)
                .show(),
            MessageDialogResult::Yes
        )
    }
    fn try_read_event() -> Result<Option<Event_>> {
        if poll(Duration::from_millis(100))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    fn handle_event(event: &Event_, terminal_tx: &Sender<TerminalEvent>) -> bool {
        match event {
            Event_::FocusGained
            | Event_::FocusLost
            | Event_::Key(KeyEvent_ {
                kind: KeyEventKind::Release | KeyEventKind::Repeat,
                ..
            }) => {}
            Event_::Key(KeyEvent_ {
                code: KeyCode_::Char('c'),
                modifiers: KeyModifiers_::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if Self::confirm(QUIT_TITLE, QUIT_MESSAGE) {
                    _ = terminal_tx.send(TerminalEvent::Closed);
                    return false;
                }
            }
            Event_::Key(KeyEvent_ {
                code: KeyCode_::Char('l'),
                modifiers: KeyModifiers_::CONTROL,
                kind: KeyEventKind::Press,
                ..
            }) => {
                _ = terminal_tx.send(TerminalEvent::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                }));
            }
            Event_::Key(KeyEvent_ {
                code: KeyCode_::Char(c),
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) if *modifiers == KeyModifiers_::NONE || *modifiers == KeyModifiers_::SHIFT => {
                _ = terminal_tx.send(TerminalEvent::Key(KeyEvent {
                    code: KeyCode::Char(*c),
                    modifiers: KeyModifiers::NONE,
                }));
            }
            Event_::Key(KeyEvent_ {
                code:
                    key_code
                    @ (KeyCode_::Backspace | KeyCode_::Delete | KeyCode_::Esc | KeyCode_::Enter),
                modifiers: KeyModifiers_::NONE,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if let Some(code) = translate_key_code(*key_code) {
                    _ = terminal_tx.send(TerminalEvent::Key(KeyEvent {
                        code,
                        modifiers: KeyModifiers::NONE,
                    }));
                } else {
                    warn!("unimplemented key code: {key_code:?}");
                }
            }
            _ => warn!("unhandled event: {event:?}"),
        }
        true
    }
}

impl Terminal for TextTerminal {
    fn backspace(&self) {
        print!("\x08 \x08");
        stdout().flush().unwrap();
    }

    fn clear_line(&self) {
        _ = stdout().execute(Clear(ClearType::CurrentLine)).unwrap();
        print!("\r");
        stdout().flush().unwrap();
    }

    fn clear_screen(&self) {
        _ = stdout().execute(Clear(ClearType::All)).unwrap();
        _ = stdout().execute(MoveTo(0, 0)).unwrap();
    }

    fn new_line(&self) {
        println!("\r");
        stdout().flush().unwrap();
    }

    fn request_quit(&self) {
        _ = self.internal_tx.send(InternalEvent::Quit);
    }

    fn write<S: Into<String>>(&self, s: S) {
        let s = s.into();
        *self.column.borrow_mut() += s.len();
        print!("{s}");
        stdout().flush().unwrap();
    }
}
