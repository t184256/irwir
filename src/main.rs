// Copyright (c) 2018 Alexander Sosedkin <monk@unboiled.info>

// irwir is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// irwir is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with irwir.  If not, see <http://www.gnu.org/licenses/>.

extern crate evdev_rs;
extern crate input_linux;
extern crate ref_slice;
use std::fs::File;
use input_linux::{EvdevHandle, UInputHandle};
use input_linux::{InputEvent, Event, KeyEvent};

// I don't know a better way to create events from strings
// https://github.com/arcnmx/input-linux-rs/issues/1
fn key_from_name(s: &str) -> input_linux::Key {
    let ec = evdev_rs::enums::EventCode::from_str(&evdev_rs::enums::EventType::EV_KEY, s).unwrap();
    let code = evdev_rs::util::event_code_to_int(&ec).1;
    input_linux::Key::from_code(code as u16).unwrap()
}


fn main() {
    let device_path = String::from("/dev/input/by-path/platform-i8042-serio-0-event-kbd");
    println!("{}", device_path);

    let abort_key_name = "KEY_PAUSE";
    let abort_key = key_from_name(abort_key_name);

    let in_fd = File::open(device_path).unwrap();
    let in_dev = EvdevHandle::new(&in_fd);
    in_dev.grab(true).unwrap();

    let ui_fd = File::create("/dev/uinput").unwrap();
    let ui_dev = UInputHandle::new(&ui_fd);
    ui_dev.set_evbit(input_linux::EventKind::Key).unwrap();
    ui_dev.set_keybit(input_linux::Key::KeyA).unwrap();
    ui_dev.set_keybit(input_linux::Key::KeyB).unwrap();
    ui_dev.create(&input_linux::InputId::default(), b"test", 0, &[]).unwrap();

    println!("Events:");
    loop {
        // TODO: am I overcomplicating things?
        let mut input_event = unsafe { std::mem::zeroed() };
        let a = in_dev.read(ref_slice::ref_slice_mut(&mut input_event));
        if let Ok(1) = a {
            let ev = *InputEvent::from_raw(&input_event).unwrap();
            let ev = Event::new(ev).unwrap();
            match ev {
                Event::Key(KeyEvent { key, value, .. }) if key == abort_key && value == 0 => break,
                _ => {
                    //println!("{:?}", ev);
                    ui_dev.write(ref_slice::ref_slice(&ev.as_ref().as_raw())).unwrap();
                }
            }
        } else {
            // TODO: more idiomatic error handling? match costs indentation
            panic!();
        }
    }
}
