use crate::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn play() {
    if let Err(error) = listen(choose_play_file) {
        println!("Error: {:?}", error)
    }
}

fn choose_play_file(event: Event) {
    match event.event_type {
        KeyRelease(key) => {

            if let Ok(lines) = read_lines(letter_matcher(key)) {
                for line in lines {
                    matcher(line.unwrap())
                }
            }
            panic!("Macro played.")
        }
        _ => {}
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
