use super::keylogger::Keylogger;

use std::{
    fs,
    thread::sleep,
    time::Duration,
};

const DEVICES_PATH: &str = "/proc/bus/input/devices";
const DATA_PATH: &str = "/tmp/data";
const EV_MASK: i32 = 1206;

pub struct Linux {
}

impl Keylogger for Linux {
    fn find_keyboard(&self) -> bool {
        true
        // let delay = Duration::from_millis(500);

        // loop {
        //     let contents = match fs::read_to_string(DEVICES_PATH) {
        //         Ok(c) => c,
        //         Err(_) => {
        //             sleep(delay);
        //             continue; // try to open again
        //         }
        //     };
        // }
    }

    fn start_listening(&self) {

    }

    fn stop_listening(&self) {

    }

    fn log_keystroke(&self) -> char {
        'h'
    }
}
