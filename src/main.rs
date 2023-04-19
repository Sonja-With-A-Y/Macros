use std::fs::{File, OpenOptions};
use std::io::Write;

use rdev::EventType::{KeyPress, KeyRelease};
use rdev::{listen, Event, EventType};

mod play;
pub use play::*;
fn main() {
    if let Err(error) = listen(idle) {
        println!("Error: {:?}", error)
    }
}

fn idle(event: Event) {
    match event.name {
        Some(string) => if string == "q" {record()} else if string == "p" {play()},
        _ => (),
    }
}

fn record() {
    File::create("a.txt").expect("File creation failed");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {

    let f = OpenOptions::new()
        .append(true)
        .open("a.txt")
        .expect("Cannot open file");

    match event.event_type {

        KeyPress(_) => log(f, event.event_type),
        KeyRelease(_) => log(f,  event.event_type),
        _ => {}
    };
}

fn log(mut f: File, pressed_or_released: EventType) {
    f.write(format!("{:?}\n", pressed_or_released).as_bytes()).expect("idk");
}
