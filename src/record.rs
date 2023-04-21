use crate::*;

pub fn record() {
    if let Err(error) = listen(choose_file) {
        println!("Error: {:?}", error)
    }
}

fn choose_file(event: Event) {
    match event.event_type {
        KeyRelease(_) => {
            File::create("a.txt").expect("File creation failed");

            if let Err(error) = listen(recording) {
                println!("Error: {:?}", error)
            }
        }
        _ => {}
    }
}

fn recording(event: Event) {
    let f = OpenOptions::new()
        .append(true)
        .open("a.txt")
        .expect("Cannot open file");

    match event.event_type {
        KeyPress(F1) => panic!("Macro recorded successfully."),
        KeyPress(_) => log(f, event.event_type),
        KeyRelease(_) => log(f, event.event_type),
        _ => {}
    };
}

fn log(mut f: File, pressed_or_released: EventType) {
    f.write(format!("{:?}\n", pressed_or_released).as_bytes())
        .expect("idk");
}
