use crate::*;

pub fn play() {
    let mut f = File::open("a.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let mut vector = Vec::new();

    let log_item = Regex::new(r"(Press|Release).*(Key.*)\)").unwrap();

    for cap in log_item.captures_iter(&content) {
        vector.push((cap[1].to_owned(), cap[2].to_owned()))
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
