#[cfg(feature = "wozmon")]
use getch_rs::{Getch, Key};
use r6502lib::MemoryView;
#[cfg(feature = "wozmon")]
use std::io::{stdout, Write};
#[cfg(feature = "wozmon")]
use std::thread::scope;

// Apple I PIA addresses etc.
#[allow(unused)]
pub(crate) const KBD: u16 = 0xD010; // PIA.A keyboard input
#[allow(unused)]
pub(crate) const KBDCR: u16 = 0xD011; // PIA.A keyboard control register
pub(crate) const DSP: u16 = 0xD012; // PIA.B display output register
#[allow(unused)]
pub(crate) const DSPCR: u16 = 0xD013; //  PIA.B display control register

#[cfg(not(feature = "wozmon"))]
pub(crate) fn run_pia(_memory: MemoryView) {}

#[cfg(feature = "wozmon")]
pub(crate) fn run_pia(memory: MemoryView) {
    fn set_kbd(m: &MemoryView, c: char) {
        let c = c.to_ascii_uppercase();
        let char_value = c as u8;
        let kbd = char_value | 0x80;
        m.store(KBD, kbd);
        m.store(KBDCR, 0x80);
    }

    scope(|scope| {
        let m = memory.clone();
        _ = scope.spawn(move || {
            let g = Getch::new();
            loop {
                match g.getch() {
                    Ok(Key::Char(c)) => _ = set_kbd(&m, c),
                    Ok(Key::Delete) => _ = set_kbd(&m, '_'),
                    Ok(Key::Esc) => _ = set_kbd(&m, 0x1b as char),
                    Ok(Key::Ctrl('c')) => break,
                    _ => todo!(),
                }
            }
        });

        let mut stdout = stdout();

        loop {
            let dsp = memory.load(DSP);
            if (dsp & 0x80) != 0 {
                let char_value = dsp & 0x7f;
                if char_value == 13 {
                    println!();
                } else {
                    let c = char_value as char;
                    print!("{c}");
                }
                stdout.flush().expect("Must succeed");
                memory.store(DSP, char_value);
            }
        }
    });
}
