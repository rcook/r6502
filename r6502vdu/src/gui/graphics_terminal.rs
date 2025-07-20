use crate::constants::{QUIT_MESSAGE, QUIT_TITLE};
use crate::font::Font;
use crate::gui::sdl_key_util::{normalize_keymod, translate_key_code, translate_key_modifiers};
use crate::gui::{Screen, StateManager, confirm};
use crate::terminal::Terminal;
use crate::terminal_command::TerminalCommand;
use crate::terminal_event::TerminalEvent;
use crate::util::{Channel, assert_is_main_thread};
use anyhow::{Result, anyhow};
use log::{info, warn};
use path_absolutize::Absolutize;
use r6502lib::keyboard::{KeyCode, KeyEvent, KeyModifiers};
use sdl3::Sdl;
use sdl3::event::{Event as Event_, EventSender, WindowEvent};
use sdl3::keyboard::{Keycode, Mod};
use sdl3::timer::add_timer;
use sdl3::ttf::Sdl3TtfContext;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::scope;
use std::time::Duration;

const CURSOR_INTERVAL: Duration = Duration::from_millis(250);

pub struct GraphicsTerminal {
    event_sender: EventSender,
}

impl GraphicsTerminal {
    pub fn with<F, T>(font: &Font, action: F) -> Result<T>
    where
        F: FnOnce(&GraphicsTerminal, &Receiver<TerminalEvent>) -> Result<T> + Send + Sync,
        T: Send,
    {
        scope(|scope| -> Result<T> {
            info!("graphics terminal thread started");

            assert_is_main_thread();

            let state_manager = StateManager::new(&Path::new("r6502vdu.json").absolutize()?)?;

            let sdl = sdl3::init()?;
            let ttf = sdl3::ttf::init()?;

            let ev = sdl.event()?;
            ev.register_custom_event::<TerminalCommand>()?;

            let terminal = GraphicsTerminal {
                event_sender: ev.event_sender(),
            };
            let terminal_channel = Channel::new();
            let action_handle = scope.spawn(move || action(&terminal, &terminal_channel.rx));

            let event_sender = ev.event_sender();
            let delay = u32::try_from(CURSOR_INTERVAL.as_millis())?;
            let timer = add_timer(
                delay,
                Box::new({
                    move || {
                        _ = event_sender.push_custom_event(TerminalCommand::UpdateCursor);
                        delay
                    }
                }),
            );

            Self::pump_events(state_manager, &terminal_channel.tx, &sdl, &ttf, font)?;

            drop(timer);
            _ = terminal_channel.tx.send(TerminalEvent::Closed);
            action_handle
                .join()
                .map_err(|e| anyhow!("action failed: {e:?}"))?
        })
    }

    #[allow(clippy::too_many_lines)]
    fn pump_events(
        mut state_manager: StateManager,
        terminal_tx: &Sender<TerminalEvent>,
        sdl: &Sdl,
        ttf: &Sdl3TtfContext,
        font: &Font,
    ) -> Result<()> {
        let mut screen = Screen::new(sdl, ttf, 40, 25, 1, state_manager.state(), font)?;
        screen.show()?;

        let mut event_pump = sdl.event_pump()?;
        for event in event_pump.wait_iter() {
            match event {
                Event_::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    ..
                } => {
                    if !Self::handle_key_down(keycode, keymod, terminal_tx, &mut screen)? {
                        break;
                    }
                }
                Event_::Quit { .. } => {
                    if confirm(Some(screen.window()), QUIT_TITLE, QUIT_MESSAGE)? {
                        _ = terminal_tx.send(TerminalEvent::Closed);
                        break;
                    }
                }
                Event_::TextInput { text, .. } => {
                    _ = terminal_tx.send(TerminalEvent::TextInput(text));
                }
                Event_::Window {
                    win_event: WindowEvent::Moved(_, _),
                    ..
                } => {
                    state_manager.update(screen.window())?;
                }
                Event_::Window {
                    win_event: WindowEvent::Resized(_, _),
                    ..
                } => {
                    state_manager.update(screen.window())?;
                    screen.show()?;
                }
                Event_::ClipboardUpdate { .. }
                | Event_::Display { .. }
                | Event_::KeyUp { .. }
                | Event_::MouseButtonDown { .. }
                | Event_::MouseButtonUp { .. }
                | Event_::MouseMotion { .. }
                | Event_::MouseWheel { .. }
                | Event_::RenderTargetsReset { .. }
                | Event_::TextEditing { .. }
                | Event_::Unknown { .. }
                | Event_::Window {
                    win_event:
                        WindowEvent::CloseRequested
                        | WindowEvent::DisplayChanged(_)
                        | WindowEvent::MouseEnter
                        | WindowEvent::Exposed
                        | WindowEvent::FocusGained
                        | WindowEvent::FocusLost
                        | WindowEvent::Maximized
                        | WindowEvent::MouseLeave
                        | WindowEvent::PixelSizeChanged(_, _)
                        | WindowEvent::Restored
                        | WindowEvent::Shown,
                    ..
                } => {}
                e if e.is_user_event() => {
                    match e
                        .as_user_event_type::<TerminalCommand>()
                        .ok_or_else(|| anyhow!("could not get custom event"))?
                    {
                        TerminalCommand::Backspace => screen.backspace()?,
                        TerminalCommand::ClearLine => screen.clear_line()?,
                        TerminalCommand::ClearScreen => screen.clear_screen()?,
                        TerminalCommand::NewLine => screen.new_line()?,
                        TerminalCommand::UpdateCursor => screen.update_cursor()?,
                        TerminalCommand::Write(s) => screen.write(&s)?,
                    }
                }
                _ => warn!("unhandled event: {event:?}"),
            }
        }
        Ok(())
    }

    fn handle_key_down(
        keycode: Keycode,
        keymod: Mod,
        terminal_tx: &Sender<TerminalEvent>,
        screen: &mut Screen,
    ) -> Result<bool> {
        match (keycode, normalize_keymod(keymod)) {
            (Keycode::C, Mod::LCTRLMOD) => {
                if confirm(Some(screen.window()), QUIT_TITLE, QUIT_MESSAGE)? {
                    _ = terminal_tx.send(TerminalEvent::Closed);
                    return Ok(false);
                }
            }
            (Keycode::L, Mod::LCTRLMOD) => {
                _ = terminal_tx.send(TerminalEvent::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::CONTROL,
                }));
            }
            (Keycode::Backspace | Keycode::Delete | Keycode::Escape, Mod::NOMOD) => {
                if let Some(code) = translate_key_code(keycode) {
                    _ = terminal_tx.send(TerminalEvent::Key(KeyEvent {
                        code,
                        modifiers: KeyModifiers::NONE,
                    }));
                } else {
                    warn!("unimplemented key code: {keycode:?}");
                }
            }
            (Keycode::F11, Mod::NOMOD) | (Keycode::F, Mod::LGUIMOD) => {
                screen.toggle_full_screen()?;
            }
            (Keycode::Return, keymod) => {
                if let Some(code) = translate_key_code(keycode) {
                    _ = terminal_tx.send(TerminalEvent::Key(KeyEvent {
                        code,
                        modifiers: translate_key_modifiers(keymod),
                    }));
                } else {
                    warn!("unimplemented key code: {keycode:?}");
                }
            }
            _ => {}
        }
        Ok(true)
    }
}

impl Terminal for GraphicsTerminal {
    fn backspace(&self) {
        _ = self
            .event_sender
            .push_custom_event(TerminalCommand::Backspace);
    }

    fn clear_line(&self) {
        _ = self
            .event_sender
            .push_custom_event(TerminalCommand::ClearLine);
    }

    fn clear_screen(&self) {
        _ = self
            .event_sender
            .push_custom_event(TerminalCommand::ClearScreen);
    }

    fn new_line(&self) {
        _ = self
            .event_sender
            .push_custom_event(TerminalCommand::NewLine);
    }

    fn request_quit(&self) {
        _ = self.event_sender.push_event(Event_::Quit { timestamp: 0 });
    }

    fn write<S: Into<String>>(&self, s: S) {
        _ = self
            .event_sender
            .push_custom_event(TerminalCommand::Write(s.into()));
    }
}
