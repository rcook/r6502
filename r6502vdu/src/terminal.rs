use r6502lib::ascii::{CR, DEL, ESC};

pub trait Terminal {
    fn backspace(&self);

    fn clear_line(&self);

    fn clear_screen(&self);

    fn new_line(&self);

    fn request_quit(&self);

    fn write<S: Into<String>>(&self, s: S);

    fn write_decode<S: Into<String>>(&self, s: S) {
        for c in s.into().chars() {
            if c == '\n' {
                self.new_line();
            } else if c == '\r' {
                // Ignore
            } else if c as u8 == DEL {
                self.backspace();
            } else if c as u8 == ESC {
                // Ignore
            } else if c.is_ascii_control() {
                self.write(format!("[{value}]", value = c as u8))
            } else if c.is_ascii() {
                self.write(c)
            } else {
                self.write(format!("[out of range {value}]", value = c as u8))
            }
        }
    }
}
