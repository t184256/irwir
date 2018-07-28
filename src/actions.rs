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

use enums_from_names::parse_key;
use input_linux::{InputEvent, KeyEvent};
use scripting::ScriptingEngine;
use uinput_device::UInputDevice;

pub type Tag = String;

#[derive(VmType, Getable, Pushable, Debug, PartialEq)]
#[gluon(vm_type = "MapToKey")]
pub struct MapToKey {
    tag: Tag,
}

impl MapToKey {
    pub fn new(t: Tag) -> MapToKey {
        MapToKey { tag: t }
    }
    pub fn execute(
        &self,
        parent_event: InputEvent,
        uinput_device: &UInputDevice,
        _: &ScriptingEngine,
    ) {
        let InputEvent { time, value, .. } = parent_event;
        let key = parse_key(&self.tag).unwrap();
        let event = KeyEvent::new(time, key, value);
        uinput_device.simulate(InputEvent::from(event)); // could be nicer?
    }
}
