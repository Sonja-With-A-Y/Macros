use crate::*;


pub fn play() {
    let mut f = File::open("a.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let mut vector = Vec::new();

    let pressed = Regex::new(r"(Press|Release).*(Key.*)\)").unwrap();

    for cap in pressed.captures_iter(&content) {
        vector.push((&cap[1], &cap[2]))
    }

    println!("{:?}", vector);

    fn send(event_type: &EventType) {
        let delay = time::Duration::from_millis(20);
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }

        thread::sleep(delay);
    }

    //send(&EventType::KeyPress(Key::KeyS));
    //send(&EventType::KeyRelease(Key::KeyS));
}
