use rdev::{simulate, EventType, SimulateError};
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::{thread, time};

use rdev::EventType::{KeyPress, KeyRelease};
use rdev::{listen, Event};

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
    match event.name {
        Some(string) => {
            if string == "q" {
                record()
            } else if string == "p" {
                play()
            }
        }
        _ => (),
    }
}
