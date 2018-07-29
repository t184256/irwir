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

use enums_from_names::{parse_abs, parse_event_kind, parse_key, parse_rel, parse_syn};
use input_linux::{
    AbsoluteEvent, EventKind, InputEvent, KeyEvent, RelativeEvent, SynchronizeEvent,
};
use scripting::ScriptingEngine;
use uinput_device::UInputDevice;

// I chose enum-style 'polymorphism' for the ease of traversal of the gluon boundary

#[derive(VmType, Getable, Pushable, Debug, PartialEq)]
#[gluon(vm_type = "Action")]
pub enum Action {
    Event {
        kind_name: String,
        code_name: String,
        value: i32,
    },
    Key(String),
}

impl Action {
    pub fn execute(
        &self,
        parent_event: InputEvent,
        uinput_device: &UInputDevice,
        _: &ScriptingEngine,
    ) {
        let InputEvent { time, value, .. } = parent_event;
        match self {
            Action::Key(key_name) => {
                let key = parse_key(&key_name).unwrap();
                let event = KeyEvent::new(time, key, value);
                uinput_device.simulate(InputEvent::from(event)); // could be nicer?
            }
            Action::Event {
                kind_name,
                code_name,
                value,
            } => {
                let kind = parse_event_kind(kind_name).unwrap();
                // TODO: make nicer?
                match kind {
                    EventKind::Synchronize => uinput_device.simulate(InputEvent::from(
                        SynchronizeEvent::new(time, parse_syn(code_name).unwrap(), *value),
                    )),
                    EventKind::Key => uinput_device.simulate(InputEvent::from(KeyEvent::new(
                        time,
                        parse_key(code_name).unwrap(),
                        *value,
                    ))),
                    EventKind::Relative => uinput_device.simulate(InputEvent::from(
                        RelativeEvent::new(time, parse_rel(code_name).unwrap(), *value),
                    )),
                    EventKind::Absolute => uinput_device.simulate(InputEvent::from(
                        AbsoluteEvent::new(time, parse_abs(code_name).unwrap(), *value),
                    )),
                    // TODO: add more
                    _ => uinput_device.simulate(InputEvent {
                        time,
                        kind,
                        code: code_name.parse::<u16>().unwrap(),
                        value: *value,
                    }),
                };
            }
        }
    }
}
