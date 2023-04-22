use crate::*;

pub fn record() {
    if let Err(error) = listen(choose_memory_slot) {
        println!("Error: {:?}", error)
    }
}

fn choose_memory_slot(event: Event) {
    match event.event_type {
        KeyRelease(key) => {

            let mut temp_to = OpenOptions::new()
                .write(true)
                .open("temp_to.txt")
                .expect("temp_to problem");

            //TODO Something here is adding a new line at the end of the file being created and it
            //shouldn't be.

            temp_to.write(memory_slot_matcher(key).as_bytes()).expect("Problem writing to temp_to");

            File::create(memory_slot_matcher(key)).expect("File creation failed");

            let temp = OpenOptions::new()
                .write(true)
                .open("temp.txt")
                .expect("Problem opening temp.txt to clear it");

            temp.set_len(0).expect("Problem clearing temp.txt");
            

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
        .open("temp.txt")
        .expect("Cannot open file");

    match event.event_type {
        KeyPress(ControlRight) => panic!("Macro recorded successfully."),
        KeyPress(_) => write_to_temp(f, event.event_type),
        KeyRelease(_) => write_to_temp(f, event.event_type),
        _ => {}
    };
}

fn write_to_temp(mut f: File, pressed_or_released: EventType) {
    f.write(format!("{:?}\n", pressed_or_released).as_bytes()).expect("idk");
}

pub fn memory_slot_matcher(key: Key) -> String {
    match key { 
        KeyQ => "q.txt".to_owned(),
        KeyW => "w.txt".to_owned(),
        KeyE => "e.txt".to_owned(),
        KeyR => "r.txt".to_owned(),
        KeyT => "t.txt".to_owned(),
        KeyY => "y.txt".to_owned(),
        KeyU => "u.txt".to_owned(),
        KeyI => "i.txt".to_owned(),
        KeyO => "o.txt".to_owned(),
        KeyP => "p.txt".to_owned(),
        KeyA => "a.txt".to_owned(),
        KeyS => "s.txt".to_owned(),
        KeyD => "d.txt".to_owned(),
        KeyF => "f.txt".to_owned(),
        KeyG => "g.txt".to_owned(),
        KeyH => "h.txt".to_owned(),
        KeyJ => "j.txt".to_owned(),
        KeyK => "k.txt".to_owned(),
        KeyL => "l.txt".to_owned(),
        KeyZ => "z.txt".to_owned(),
        KeyX => "x.txt".to_owned(),
        KeyC => "c.txt".to_owned(),
        KeyV => "v.txt".to_owned(),
        KeyB => "b.txt".to_owned(),
        KeyN => "n.txt".to_owned(),
        KeyM => "m.txt".to_owned(),
        _ => "bad.txt".to_owned()
    }
}
