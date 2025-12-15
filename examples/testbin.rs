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

fn main() {
    let mut bst = BSTMap::<i32, String>::new();
    bst.insert(10, "test".to_string());
    bst.insert(-25, "test2".to_string());
    bst.insert(4, "hello".to_string());
    bst.insert(3, "hi".to_string());
    bst.insert(15, "is it working".to_string());
    bst.insert(-30, "negatives".to_string());
    bst.insert(-29, "negatives2".to_string());
    bst.insert(102, "test".to_string());
    bst.insert(15, "123".to_string());
    bst.insert(30, "strings as values".to_string());
    bst.insert(2, "heyhy".to_string());
    bst.insert(16, "utf8żółć".to_string());

    println!("PRINTING TREE");
    bst.pretty_print();

    let test1 = 15;
    println!("BST has {} {}", test1, bst.contains(test1));

    let test2 = 100;
    println!("BST has {} {}", test2, bst.contains(test2));

    bst.remove(10);
    bst.remove(4);

    println!("PRINTING TREE AFTER REMOVAL");
    bst.pretty_print();

    {
        let bst_ref = bst.get_mut(16);

        if bst_ref.is_some() {
            let bst_ref = bst_ref.unwrap();
            *bst_ref = "utf8żółćmodified".to_string();
        }
    }

    bst = [
        (-35, "Hello!"),
        (-21, "test123"),
        (-10, "aaaa"),
        (-19, " "),
        (-40, "aasdasdasd"),
        (20, "utf8żółc"),
    ]
    .map(|(k, v)| (k, v.to_string()))
    .into();

    println!("PRINTING TREE AFTER get_mut()");
    bst.pretty_print();

    for (key, value) in bst.iter() {
        println!("Mapping: {}: \"{}\"", key, value);
    }

    bst.extend(
        [
            (99, "large number"),
            (-12, "random insert in the middle"),
            (4, "hi"),
        ]
        .map(|(k, v)| (k, v.to_string())),
    );

    println!("PRINTING TREE AFTER extend()");
    bst.pretty_print();

    for (key, value) in bst.iter_breadthfirst() {
        println!("Mapping BFS: {}: \"{}\"", key, value);
    }

    for (_, value) in bst.iter_inorder_mut() {
        value.insert(0, 'A');
    }

    println!("PRINTING TREE AFTER iter_mut()");
    bst.pretty_print();

    for (key, value) in bst {
        println!("Owned mapping: {}: \"{}\"", key, value);
    }
}
