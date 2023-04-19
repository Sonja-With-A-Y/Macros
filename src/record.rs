use crate::*;

pub fn record() {
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
        KeyRelease(_) => log(f, event.event_type),
        _ => {}
    };
}

fn log(mut f: File, pressed_or_released: EventType) {
    f.write(format!("{:?}\n", pressed_or_released).as_bytes())
        .expect("idk");
}
