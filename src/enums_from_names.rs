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

use input_linux::{AbsoluteAxis, EventKind, Key, RelativeAxis, SynchronizeKind};
use ron;
use std::error;

pub fn parse_syn(name: &str) -> Result<SynchronizeKind, Box<error::Error>> {
    Ok(ron::de::from_str(&name)?)
}

pub fn parse_key(name: &str) -> Result<Key, Box<error::Error>> {
    Ok(ron::de::from_str(&name)?)
}

pub fn parse_rel(name: &str) -> Result<RelativeAxis, Box<error::Error>> {
    Ok(ron::de::from_str(&name)?)
}

pub fn parse_abs(name: &str) -> Result<AbsoluteAxis, Box<error::Error>> {
    Ok(ron::de::from_str(&name)?)
}

// TODO: add more

pub fn parse_event_kind(kind: &str) -> Result<EventKind, Box<error::Error>> {
    let kind: EventKind = ron::de::from_str(&kind)?;
    Ok(kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_key_test() {
        let key = parse_key("KeyA").unwrap();
        assert_eq!(key, Key::KeyA);
    }
    #[test]
    fn parse_event_kind_test() {
        let kind = parse_event_kind("Key").unwrap();
        assert_eq!(kind, EventKind::Key);
    }
}
