use anyhow::Result;
use cursive::backends::crossterm::crossterm::cursor::MoveTo;
use cursive::backends::crossterm::crossterm::style::{Color, SetForegroundColor};
use cursive::backends::crossterm::crossterm::terminal::{Clear, ClearType};
use cursive::backends::crossterm::crossterm::QueueableCommand;
use std::collections::HashMap;
use std::io::Stdout;
use std::io::Write;
use std::sync::LazyLock;

pub type VduCode = (
    u8,
    &'static str,
    u8,
    &'static str,
    Option<fn(&mut Stdout, &[u8])>,
);

pub static VDU_CODES_BY_CODE: LazyLock<HashMap<u8, VduCode>> =
    LazyLock::new(|| VDU_CODES.into_iter().map(|c| (c.0, c)).collect());

const VDU_CODES: [VduCode; 33] = [
    (0, "@", 0, "Does nothing", None),
    (1, "A", 1, "Send next character to printer only", None),
    (2, "B", 0, "Enable printer", None),
    (3, "C", 0, "Disable printer", None),
    (4, "D", 0, "Write text at text cursor", None),
    (5, "E", 0, "Write text at graphics cursor", None),
    (6, "F", 0, "Enable VDU drivers", None),
    (7, "G", 0, "Make short beep (BEL)", None),
    (8, "H", 0, "Move cursor back one character", None),
    (9, "I", 0, "Move cursor forward one character", None),
    (10, "J", 0, "Move cursor down one line", None),
    (11, "K", 0, "Move cursor up one line", None),
    (12, "L", 0, "Clear text area", Some(clear_text_area)),
    (13, "M", 0, "Carriage return", None),
    (14, "N", 0, "Paged mode on", None),
    (15, "O", 0, "Paped mode off", None),
    (16, "P", 0, "Clear graphics area", None),
    (17, "Q", 1, "Define text colour", Some(define_text_colour)),
    (18, "R", 2, "Define graphics colour", None),
    (19, "S", 5, "Define logical colour", None),
    (20, "T", 0, "Restore default logical colours", None),
    (21, "U", 0, "Disable VDU drivers or delete input line", None),
    (22, "V", 1, "Select screen MODE", None),
    (
        23,
        "W",
        9,
        "Re-program display character + various other fn's",
        None,
    ),
    (24, "X", 8, "Define graphics window", None),
    (25, "Y", 5, "PLOT k,x,y", None),
    (26, "Z", 0, "Restore default windows", None),
    (27, "[", 0, "ESCAPE value", None),
    (28, "\\", 4, "Define text window", None),
    (29, "]", 4, "Define graphics origin", None),
    (30, "^", 0, "Home text cursor to top left of window", None),
    (31, "_", 2, "Move text cursor to x, y", None),
    (127, "del", 0, "Backspace and delete", None),
];

fn define_text_colour(stdout: &mut Stdout, _args: &[u8]) {
    fn inner(stdout: &mut Stdout) -> Result<()> {
        stdout.queue(SetForegroundColor(Color::Red))?;
        stdout.flush()?;
        Ok(())
    }

    inner(stdout).unwrap();
}

fn clear_text_area(stdout: &mut Stdout, _args: &[u8]) {
    fn inner(stdout: &mut Stdout) -> Result<()> {
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        stdout.flush()?;
        Ok(())
    }

    inner(stdout).unwrap();
}
