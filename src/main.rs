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

extern crate gluon;
#[macro_use]
extern crate gluon_codegen;
#[macro_use]
extern crate gluon_vm;
#[macro_use]
extern crate indoc;
extern crate input_linux;
#[macro_use]
extern crate log;
extern crate ref_slice;
extern crate ron;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use input_linux::EvdevHandle;
use input_linux::{Event, InputEvent, KeyEvent};
use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::prelude::*;

mod actions;
mod enums_from_names;
mod scripting;
use scripting::{IrwirGluonFunc, ScriptingEngine};
mod uinput_device;
use uinput_device::UInputDevice;

pub type Tag = String;

#[derive(Debug, Serialize, Deserialize)]
struct IrwirConfig {
    device_path: String,
    abort_key: input_linux::Key,
    exported_keys: Vec<input_linux::Key>,
    layout: HashMap<Tag, input_linux::Key>,
    map: HashMap<Tag, String>,
}

fn read_config(fname: &str) -> Result<IrwirConfig, Box<error::Error>> {
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let cfg: IrwirConfig = toml::from_str(&s)?;
    Ok(cfg)
}

fn create_function_mappings<'a>(
    cfg: &IrwirConfig,
    se: &'a ScriptingEngine,
) -> HashMap<input_linux::Key, IrwirGluonFunc<'a>> {
    let mut map_to_functions = HashMap::new();
    for (tag, key) in &cfg.layout {
        if let Some(mapped_code) = cfg.map.get(tag) {
            let func = se.make_func(mapped_code);
            map_to_functions.insert(key.clone(), func);
        } else {
            warn!("Unmapped tag {}", tag);
        }
    }
    map_to_functions
}

fn irwir(config: IrwirConfig) {
    let se = ScriptingEngine::new();
    let mut map_to_functions = create_function_mappings(&config, &se);

    let in_fd = File::open(config.device_path).unwrap();
    let in_dev = EvdevHandle::new(&in_fd);
    in_dev.grab(true).unwrap();

    let ui_dev = UInputDevice::new(config.exported_keys);

    loop {
        // TODO: am I overcomplicating things?
        let mut input_event = unsafe { std::mem::zeroed() };
        let a = in_dev.read(ref_slice::ref_slice_mut(&mut input_event));
        if let Ok(1) = a {
            let iev = *InputEvent::from_raw(&input_event).unwrap();
            let ev = Event::new(iev).unwrap();
            match ev {
                Event::Key(KeyEvent { key, value, .. })
                    if key == config.abort_key && value == 0 =>
                {
                    break
                }
                Event::Key(KeyEvent { key, value, .. }) if map_to_functions.contains_key(&key) => {
                    // TODO: idiomatize lookup
                    let mut func = map_to_functions.get_mut(&key).unwrap();
                    let action = func.call(value).unwrap();
                    action.execute(iev, &ui_dev, &se);
                }
                _ => {
                    //println!("{:?}", ev);
                    ui_dev.simulate(InputEvent::from(ev)); // could be nicer?
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
