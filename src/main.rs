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
use input_linux::{InputId, EventKind};
use input_linux::{InputEvent, Event, KeyEvent, Key};
use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate serde_derive;


type Tag = String;


#[derive(Debug, Serialize, Deserialize)]
struct IrwirConfig {
    device_path: String,
    abort_key: Key,
    exported_keys: Vec<Key>,
    layout: HashMap<Tag, Key>,
    #[serde(skip)]
    inverse_layout: HashMap<Key, Tag>,
    map: HashMap<Tag, Key>,
}


fn read_config(fname: &str) -> Result<IrwirConfig, Box<error::Error>> {
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let mut cfg : IrwirConfig = toml::from_str(&s)?;
    for (tag, key) in &cfg.layout {
        cfg.inverse_layout.insert(key.clone(), tag.clone());
    }
    Ok(cfg)
}


fn irwir(config: IrwirConfig) {
    let in_fd = File::open(config.device_path).unwrap();
    let in_dev = EvdevHandle::new(&in_fd);
    in_dev.grab(true).unwrap();

    let ui_fd = File::create("/dev/uinput").unwrap();
    let ui_dev = UInputHandle::new(&ui_fd);
    ui_dev.set_evbit(EventKind::Key).unwrap();
    for exported_key in config.exported_keys {
        ui_dev.set_keybit(exported_key).unwrap();
    }
    ui_dev
        .create(&InputId::default(), b"test", 0, &[])
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
                Event::Key(KeyEvent { time, key, value, .. })
                    if config.inverse_layout.contains_key(&key) => {
                        let tag = config.inverse_layout.get(&key).unwrap();  // TODO: idiomatize
                        let remapped_key = config.map.get(tag).unwrap();  // TODO: log if missing
                        let mut remapped_event = KeyEvent::new(
                            time, *remapped_key, value
                        );
                        ui_dev
                            .write(ref_slice::ref_slice(&remapped_event.as_ref()))
                            .unwrap();
                    }
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
