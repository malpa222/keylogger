pub trait Keylogger {
    fn find_keyboard(&self) -> bool;
    fn start_listening(&self);
    fn stop_listening(&self);
    fn log_keystroke(&self) -> char;
}
