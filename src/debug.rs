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

// This file contains functionality used for debugging during development
// This will most likely not be public API

use std::fmt::Display;

use super::BSTMap;

impl<K: Display + Ord, V: Display> BSTMap<K, V> {
    // TODO: Probably remove this/change to debug-only
    // TODO: clean this code up and change it into a BFS function?
    pub fn pretty_print(&self) {
        let mut current_vector = vec![&self.head];

        let mut space_count = 35;

        while current_vector.iter().any(|node| node.is_some()) {
            print!("{}", " ".repeat(space_count));
            for node in &current_vector {
                if node.is_some() {
                    print!(
                        "{:>3}:{:>3}(bf:{:>3})(h:{:>3})   ",
                        node.as_ref().unwrap().key,
                        node.as_ref().unwrap().value,
                        node.as_ref().unwrap().balance(),
                        node.as_ref().unwrap().height
                    );
                } else {
                    print!("X   ");
                }
            }
            println!();

            let mut next_vector = vec![];
            for node in current_vector {
                if node.is_some() {
                    next_vector.push(&(node.as_ref().unwrap().left));
                    next_vector.push(&(node.as_ref().unwrap().right));
                } else {
                    next_vector.push(&None);
                    next_vector.push(&None);
                }
            }
            current_vector = next_vector;

            space_count -= 3;
        }
    }
}
