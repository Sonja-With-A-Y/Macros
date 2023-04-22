use std::fs::read_to_string;
use std::fs::copy;
use rdev::{simulate, EventType, SimulateError};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{thread, time};

use rdev::EventType::{KeyPress, KeyRelease};
use rdev::{listen, Event, Key, Key::*};

mod play;
mod record;
mod matcher;
pub use play::*;
pub use record::*;
pub use matcher::*;

fn main() {

    let mut target = read_to_string("temp_to.txt").unwrap();

    if target.ends_with('\n') { target.pop(); };

    copy("temp.txt", target).expect("Copy failed");

    if let Err(error) = listen(idle) {
        println!("Error: {:?}", error)
    }
}

fn idle(event: Event) {
    match event.event_type {
        KeyRelease(ControlRight) => record(),
        KeyRelease(ShiftRight) => play(),
        _ => (),
    }
}
