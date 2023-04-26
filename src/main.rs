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

    println!("Tab right control to record, or right shift to play.");

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
