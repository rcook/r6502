pub trait Terminal {
    fn backspace(&self);
    fn clear_line(&self);
    fn clear_screen(&self);
    fn new_line(&self);
    fn request_quit(&self);
    fn write<S: Into<String>>(&self, s: S);
}
