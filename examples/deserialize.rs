// rbst328 - Implementation of Binary Search Tree in Rust
// Copyright (C) 2025  Maciej Sawka <maciejsawka@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use rbst328::{map::BSTMap, set::BSTSet};
use serde_json::Result;

pub fn main() -> Result<()> {
    let json = "[-45,10,999]";

    let set: BSTSet<i32> = serde_json::from_str(json)?;

    println!("JSON string for Set data structure: {}", json);
    println!("Deserialized Set data structure: {:?}", set);

    let json = "{\"test\": \"great job!\", \"hello!\": \"World!\", \"żółć\": \"😎\"}";

    let map: BSTMap<String, String> = serde_json::from_str(json)?;

    println!("JSON string for Map data structure: {}", json);
    println!("Deserialized Map data structure: {:?}", map);

    Ok(())
}
