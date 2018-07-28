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

use input_linux::Key;
use ron;
use std::error;

pub fn parse_key(name: &str) -> Result<Key, Box<error::Error>> {
    let key: Key = ron::de::from_str(&name)?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_key_test() {
        let key = parse_key("KeyA").unwrap();
        assert_eq!(key, Key::KeyA);
    }
}
