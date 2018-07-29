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
    AbsoluteAxis, AbsoluteEvent, EventKind, InputEvent, KeyEvent, RelativeEvent, SynchronizeEvent,
    SynchronizeKind,
};
use scripting::ScriptingEngine;
use uinput_device::UInputDevice;

// I chose enum-style 'polymorphism' for the ease of traversal of the gluon boundary

#[derive(VmType, Getable, Pushable, Debug, PartialEq)]
#[gluon(vm_type = "Action")]
pub enum Action {
    Combo(Vec<Action>),
    Nothing,
    Event {
        kind_name: String,
        code_name: String,
        value: i32,
    },
    Syn,
    Key(String),
    Button(String),
    Press(String),
    Release(String),
    Repeat(String),
    PressRelease(String),
    Abs(String, i32),
    Rel(String, i32),
    XY(i32, i32),
}

impl Action {
    pub fn execute(
        &self,
        parent_event: InputEvent,
        uinput_device: &UInputDevice,
        scripting_engine: &ScriptingEngine,
    ) {
        let InputEvent { time, value, .. } = parent_event;
        let simulate_key = |key_name: &String, value| {
            uinput_device.simulate(InputEvent::from(KeyEvent::new(
                time,
                parse_key(&key_name).unwrap(),
                value,
            )))
        };
        match self {
            Action::Combo(actions) => {
                for action in actions {
                    action.execute(parent_event, uinput_device, scripting_engine);
                }
            }
            Action::Nothing => {}
            Action::Syn => uinput_device.simulate(InputEvent::from(SynchronizeEvent::new(
                time,
                SynchronizeKind::Report,
                0,
            ))),
            Action::Key(key_name) => simulate_key(key_name, value),
            Action::Button(key_name) => if value != 2 {
                simulate_key(key_name, value)
            },
            Action::Press(key_name) => simulate_key(key_name, 1),
            Action::Release(key_name) => simulate_key(key_name, 0),
            Action::Repeat(key_name) => simulate_key(key_name, 2),
            Action::PressRelease(key_name) => {
                let key = parse_key(&key_name).unwrap();
                uinput_device.simulate(InputEvent::from(KeyEvent::new(time, key, 1)));
                uinput_device.simulate(InputEvent::from(KeyEvent::new(time, key, 0)));
            }
            Action::Abs(abs_name, value) => {
                let abs = parse_abs(&abs_name).unwrap();
                uinput_device.simulate(InputEvent::from(AbsoluteEvent::new(time, abs, *value)));
            }
            Action::Rel(rel_name, value) => {
                let rel = parse_rel(&rel_name).unwrap();
                uinput_device.simulate(InputEvent::from(RelativeEvent::new(time, rel, *value)));
            }
            Action::XY(value_x, value_y) => {
                // hack: refresh value
                uinput_device.simulate_several(&[
                    InputEvent::from(AbsoluteEvent::new(time, AbsoluteAxis::X, *value_x + 1)),
                    InputEvent::from(AbsoluteEvent::new(time, AbsoluteAxis::Y, *value_y)),
                ]);
                uinput_device.simulate_several(&[
                    InputEvent::from(AbsoluteEvent::new(time, AbsoluteAxis::X, *value_x)),
                    InputEvent::from(AbsoluteEvent::new(time, AbsoluteAxis::Y, *value_y)),
                ]);
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
