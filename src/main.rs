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

extern crate input_linux;
extern crate ref_slice;
extern crate toml;
use input_linux::{EvdevHandle, UInputHandle};
use input_linux::{InputEvent, Event, KeyEvent};
use std::error;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;


#[derive(Debug, Serialize, Deserialize)]
struct IrwirConfig {
    device_path: String,
    abort_key: input_linux::Key,
}


fn read_config(fname: &str) -> Result<IrwirConfig, Box<error::Error>> {
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(toml::from_str(&s)?)
}


fn irwir(config: IrwirConfig) {
    let in_fd = File::open(config.device_path).unwrap();
    let in_dev = EvdevHandle::new(&in_fd);
    in_dev.grab(true).unwrap();

    let ui_fd = File::create("/dev/uinput").unwrap();
    let ui_dev = UInputHandle::new(&ui_fd);
    ui_dev.set_evbit(input_linux::EventKind::Key).unwrap();
    ui_dev.set_keybit(input_linux::Key::KeyA).unwrap();
    ui_dev.set_keybit(input_linux::Key::KeyB).unwrap();
    ui_dev
        .create(&input_linux::InputId::default(), b"test", 0, &[])
        .unwrap();

    loop {
        // TODO: am I overcomplicating things?
        let mut input_event = unsafe { std::mem::zeroed() };
        let a = in_dev.read(ref_slice::ref_slice_mut(&mut input_event));
        if let Ok(1) = a {
            let ev = *InputEvent::from_raw(&input_event).unwrap();
            let ev = Event::new(ev).unwrap();
            match ev {
                Event::Key(KeyEvent { key, value, .. })
                    if key == config.abort_key && value == 0 => break,
                _ => {
                    //println!("{:?}", ev);
                    ui_dev
                        .write(ref_slice::ref_slice(&ev.as_ref().as_raw()))
                        .unwrap();
                }
            }
        } else {
            // TODO: more idiomatic error handling? match costs indentation
            panic!();
        }
    }
}


fn main() {
    let config = read_config("config.toml");
    match config {
        Ok(c) => irwir(c),
        Err(e) => println!("Error reading config: {}", e),
    }
}
