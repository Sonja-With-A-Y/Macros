extern crate evdev_rs as evdev;
use enigo::*;

fn main() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Layout('s'));
    enigo.key_click(Key::Layout('l'));
    enigo.key_up(Key::Layout('s'));

    enigo.key_down(Key::RControl);
    enigo.key_click(Key::Layout('t'));
    enigo.key_up(Key::RControl);
}
