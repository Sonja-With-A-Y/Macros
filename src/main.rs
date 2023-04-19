use rdev::{simulate, EventType, SimulateError};
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{thread, time};

use rdev::EventType::{KeyPress, KeyRelease};
use rdev::{listen, Event, Key::*};

mod play;
mod record;
pub use play::*;
pub use record::*;

fn main() {
    if let Err(error) = listen(idle) {
        println!("Error: {:?}", error)
    }
}

fn idle(event: Event) {
    match event.event_type {
        KeyRelease(KeyQ) => { record() },
        KeyRelease(KeyP) => { play() },
        _ => (),
    }
}
