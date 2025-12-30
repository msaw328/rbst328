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

use rbst328::BSTMap;

// TODO: remove this example file once unit tests feel complete enough?

// Tests whether a strictly-increasing or decreasing sequence is still balanced as a tree

fn main() {
    let mut bst = BSTMap::<u32, String>::new();

    for i in 0..15u32 {
        // check duplicates too
        bst.insert(i, "test1".to_string());
        bst.insert(i, "test2".to_string());
        bst.insert(i, "test3".to_string());
    }

    println!("PRETTY PRINT AFTER 50 INCREASING INSERTS");
    bst.pretty_print();

    bst.clear();

    println!("PRETTY PRINT AFTER CLEAR");
    bst.pretty_print();

    for i in (0..15u32).rev() {
        // check duplicates too
        bst.insert(i, "test1".to_string());
        bst.insert(i, "test2".to_string());
        bst.insert(i, "test3".to_string());
    }

    println!("PRETTY PRINT AFTER 50 DECREASING INSERTS");
    bst.pretty_print();
}
