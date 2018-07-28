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

use input_linux::{EventKind, InputEvent, InputId, Key, UInputHandle};
use ref_slice;
use std::fs::File;

// not a trait because I try to own the file descriptor
pub struct UInputDevice {
    _ui_fd: File, // unused, but owned for RAII purposes
    ui_dev: UInputHandle,
}

impl UInputDevice {
    // TODO: propagate errors
    pub fn new(keys: Vec<Key>) -> Self {
        // TODO: allow to set a name
        let ui_fd = File::create("/dev/uinput").unwrap();
        let ui_dev = UInputHandle::new(&ui_fd);
        ui_dev.set_evbit(EventKind::Key).unwrap();
        for exported_key in keys {
            ui_dev.set_keybit(exported_key).unwrap();
        }
        ui_dev.create(&InputId::default(), b"test", 0, &[]).unwrap();
        UInputDevice {
            _ui_fd: ui_fd,
            ui_dev,
        }
    }

    // TODO: propagate errors
    pub fn simulate(&self, ev: InputEvent) {
        self.ui_dev
            .write(ref_slice::ref_slice(&ev.as_ref()))
            .unwrap();
    }
}
