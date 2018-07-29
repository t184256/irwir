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

use std::collections::HashMap;
use std::fs::File;
use std::mem::transmute;

use input_linux::{
    AbsoluteInfo, AbsoluteInfoSetup, EventKind, EventTime, InputEvent, InputId, Key, RelativeAxis,
    SynchronizeEvent, SynchronizeKind, UInputHandle,
};
use ref_slice;

use enums_from_names::parse_abs;

// not a trait because I try to own the file descriptor
pub struct UInputDevice {
    _ui_fd: File, // unused, but owned for RAII purposes
    ui_dev: UInputHandle,
}

impl UInputDevice {
    // TODO: propagate errors
    pub fn new(
        name: String,
        keys: Vec<Key>,
        abs: HashMap<String, AbsInfo>,
        rel: Vec<RelativeAxis>,
    ) -> Self {
        // TODO: allow to set a name
        let ui_fd = File::create("/dev/uinput").unwrap();
        let ui_dev = UInputHandle::new(&ui_fd);
        if !keys.is_empty() {
            ui_dev.set_evbit(EventKind::Key).unwrap();
        }
        for exported_key in keys {
            ui_dev.set_keybit(exported_key).unwrap();
        }
        if !abs.is_empty() {
            ui_dev.set_evbit(EventKind::Absolute).unwrap();
        }
        let mut abs_setups = Vec::new();
        for (abs_name, abs_info) in &abs {
            let exported_abs = parse_abs(abs_name).unwrap();
            abs_setups.push(AbsoluteInfoSetup {
                axis: exported_abs,
                info: AbsoluteInfo {
                    value: abs_info.value,
                    minimum: abs_info.min,
                    maximum: if abs_info.max > 0 {
                        abs_info.max
                    } else {
                        0x8000
                    },
                    fuzz: abs_info.fuzz,
                    flat: abs_info.flat,
                    resolution: if abs_info.resolution > 0 {
                        abs_info.resolution
                    } else {
                        1
                    },
                },
            });
            ui_dev.set_absbit(exported_abs).unwrap();
            println!("{}", u16::from(exported_abs));
            //ui_dev.abs_setup(&setup).unwrap();
        }
        if !rel.is_empty() {
            ui_dev.set_evbit(EventKind::Relative).unwrap();
        }
        for exported_rel in rel {
            ui_dev.set_relbit(exported_rel).unwrap();
        }
        ui_dev
            .create(&InputId::default(), name.as_bytes(), 0, &abs_setups)
            .unwrap();
        UInputDevice {
            _ui_fd: ui_fd,
            ui_dev,
        }
    }

    // TODO: propagate errors
    pub fn simulate_no_syn(&self, ev: InputEvent) {
        self.ui_dev
            .write(ref_slice::ref_slice(&ev.as_ref()))
            .unwrap();
    }

    pub fn syn(&self, time: EventTime) {
        self.simulate_no_syn(InputEvent::from(SynchronizeEvent::new(
            time,
            SynchronizeKind::Report,
            0,
        )));
    }

    pub fn simulate(&self, ev: InputEvent) {
        self.simulate_no_syn(ev);
        self.syn(ev.time);
    }

    pub fn simulate_several(&self, evs: &[InputEvent]) {
        let event_array = unsafe { transmute(evs) };
        self.ui_dev.write(event_array).unwrap();
        if evs.len() > 0 {
            self.syn(evs[0].time);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsInfo {
    #[serde(default)]
    value: i32,
    #[serde(default)]
    min: i32,
    max: i32,
    #[serde(default)]
    fuzz: i32,
    #[serde(default)]
    flat: i32,
    #[serde(default)]
    resolution: i32,
}
