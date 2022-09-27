use super::keylogger::Keylogger;

pub struct Windows {
}

impl Keylogger for Windows {
    fn find_keyboard(&self) -> bool {
        true
    }

    fn start_listening(&self) {

    }

    fn stop_listening(&self) {

    }

    fn log_keystroke(&self) -> char {
        'h'
    }
}
