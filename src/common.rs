pub trait Common {
    fn find_keyboard(&self) -> bool;
    fn start_logging(&self);
    fn log_keystroke(&self, ev: libc::input_event);
}
