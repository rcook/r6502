use crate::terminal::Terminal;
use crate::terminal_event::TerminalEvent;
use anyhow::Result;
use log::{info, warn};
use r6502lib::keyboard::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc::Receiver;

const MAX_LINE_LEN: usize = 100;

#[allow(clippy::unnecessary_wraps)]
pub fn run_emulator<T: Terminal>(terminal: &T, rx: &Receiver<TerminalEvent>) -> Result<()> {
    fn push_str<T: Terminal>(terminal: &T, line: &mut String, s: &str) {
        let remaining = MAX_LINE_LEN - line.len();
        let s_len = s.len();
        if s_len <= remaining {
            line.push_str(s);
            terminal.write(s);
        } else {
            let s = &s[0..remaining];
            line.push_str(s);
            terminal.write(s);
            println!("BEEP!");
        }
    }

    info!("emulator thread started");

    terminal.write("r6502 Emulator 64K");
    terminal.new_line();
    terminal.new_line();
    terminal.write("BASIC");
    terminal.new_line();
    terminal.new_line();
    terminal.write(">");

    let mut lines = Vec::new();
    let mut line = String::new();
    loop {
        match rx.recv() {
            Ok(TerminalEvent::Closed) | Err(_) => break,
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            })) => {
                if !line.is_empty() {
                    line.pop();
                    terminal.backspace();
                }
            }
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            })) => {
                line.clear();
                terminal.clear_line();
                terminal.write(">");
            }
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers,
                ..
            })) if modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT => {
                match line.as_str() {
                    "" => {}
                    "*QUIT" => {
                        terminal.new_line();
                        terminal.request_quit();
                    }
                    "LIST" => {
                        for (i, line) in lines.iter().enumerate() {
                            terminal.new_line();
                            terminal.write(format!("{i:>5} {line}"));
                        }
                    }
                    _ => {
                        terminal.new_line();
                        terminal.write(format!("You entered \"{line}\""));
                        lines.push(line.clone());
                    }
                }
                line.clear();
                terminal.new_line();
                terminal.write(">");
            }
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::CONTROL,
            })) => terminal.clear_screen(),
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
            })) => push_str(terminal, &mut line, &String::from(c)),
            Ok(TerminalEvent::TextInput(s)) => push_str(terminal, &mut line, &s),
            Ok(e) => warn!("unhandled event: {e:?}"),
        }
    }

    Ok(())
}
