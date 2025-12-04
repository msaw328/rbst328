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


use std::{
    cmp::{Ord, Ordering},
    fmt::Display,
};

// Shorthand for a referece to a Box'ed node that may or may not be there
type NodeRef<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> {
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
    key: K,
    value: V,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            value,
            key,
        }
    }
}

pub struct BSTMap<K: Ord, V> {
    head: NodeRef<K, V>,
}

impl<K: Ord, V> BSTMap<K, V> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn clear(&mut self) {
        self.head = None
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut current_node = &mut self.head;

        while current_node.is_some() {
            // unwrap is safe inside the loop, since current_node is Some
            current_node = match current_node.as_ref().unwrap().key.cmp(&key) {
                Ordering::Less => &mut current_node.as_mut().unwrap().right,
                Ordering::Greater => &mut current_node.as_mut().unwrap().left,
                Ordering::Equal => break,
            }
        }

        let old_value = current_node.take();
        *current_node = Some(Box::new(Node::new(key, value)));

        match old_value {
            None => None,
            Some(node) => Some(node.value),
        }
    }

    pub fn contains(&self, key: K) -> bool {
        let mut current_node = &self.head;

        while current_node.is_some() {
            // unwrap is safe inside the loop, since current_node is Some
            current_node = match current_node.as_ref().unwrap().key.cmp(&key) {
                Ordering::Less => &current_node.as_ref().unwrap().right,
                Ordering::Greater => &current_node.as_ref().unwrap().left,
                Ordering::Equal => return true,
            }
        }

        false
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut current_node = &self.head;

        while current_node.is_some() {
            let inner = current_node.as_ref().unwrap();

            // unwrap is safe inside the loop, since current_node is Some
            current_node = match inner.key.cmp(&key) {
                Ordering::Less => &inner.right,
                Ordering::Greater => &inner.left,
                Ordering::Equal => return Some(&inner.value),
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        let mut current_node = &mut self.head;

        while current_node.is_some() {
            let inner = current_node.as_mut().unwrap();

            // unwrap is safe inside the loop, since current_node is Some
            current_node = match inner.key.cmp(&key) {
                Ordering::Less => &mut inner.right,
                Ordering::Greater => &mut inner.left,
                Ordering::Equal => return Some(&mut inner.value),
            }
        }

        None
    }

    // TODO: clean this code up, if possible
    pub fn remove(&mut self, key: K) -> Option<V> {
        // First - find current node, if it is even in there
        let mut current_node = &mut self.head;

        while current_node.is_some() {
            current_node = match current_node.as_mut().unwrap().key.cmp(&key) {
                Ordering::Less => &mut current_node.as_mut().unwrap().right,
                Ordering::Greater => &mut current_node.as_mut().unwrap().left,
                Ordering::Equal => break,
            }
        }

        // This means that key is not in the tree - no removals
        if current_node.is_none() {
            return None;
        }

        // Below cases are from the wikipedia article: https://en.wikipedia.org/wiki/Binary_search_tree#Deletion
        // Case 1 - leaf node - just remove and call it a day
        if current_node.as_ref().unwrap().right.is_none() && current_node.as_ref().unwrap().left.is_none() {
            return Some(current_node.take().unwrap().value);
        }

        // Case 2 - one child - replace parent with child
        if current_node.as_ref().unwrap().right.is_none() {
            let mut old_node = current_node.take().unwrap();
            *current_node = old_node.left.take();
            return Some(old_node.value);
        }

        if current_node.as_ref().unwrap().left.is_none() {
            let mut old_node = current_node.take().unwrap();
            *current_node = old_node.right.take();
            return Some(old_node.value);
        }

        // Case 3 - two children - search for in order successor
        // Case 3a - if in order successor is immediately the right node (right node has no left subtree)
        // then replace parent with it, while keeping the left subtree
        // At this point both children are guaranteed to be Some so unwrap is safe
        if current_node.as_ref().unwrap().right.as_ref().unwrap().left.is_none() {
            let saved_left = current_node.as_mut().unwrap().left.take(); // save left subtree of successor
            let saved_right = current_node.as_mut().unwrap().right.take();

            let old_node = current_node.take().unwrap();

            *current_node = saved_right; // replace current node with right subtree of successor
            current_node.as_mut().unwrap().left = saved_left; // append saved left subtree

            return Some(old_node.value);
        }

        // Case 3b - in order successor is not immediately the right node - search for it
        // need to keep reference to successors parent in order to replace successor
        // successor is the left child of successors parent
        // at the beginning we are guaranteed that left node exists due to earlier if(), so unwrap is safe
        let mut successors_parent = &mut current_node.as_mut().unwrap().right;
        let mut successor = &mut successors_parent.as_mut().unwrap().left;

        // While successor has a left subtree, move one level lower to the left
        while successor.as_ref().unwrap().left.is_some() {
            successors_parent = &mut successors_parent.as_mut().unwrap().left;
            successor = &mut successors_parent.as_mut().unwrap().left;
        }

        // Store inner Boxed node of successor for easier access - also take it, since we're moving it anyways
        let mut successor_inner = successor.take().unwrap();

        // Replace successors parent's left subtree with right subtree of successor
        successors_parent.as_mut().unwrap().left = successor_inner.right;

        let saved_left = current_node.as_mut().unwrap().left.take();
        let saved_right = current_node.as_mut().unwrap().right.take();
        let old_node = current_node.take().unwrap();

        // Replace removed node with successor
        successor_inner.right = saved_right;
        successor_inner.left = saved_left;
        *current_node = Some(successor_inner);

        return Some(old_node.value);
    }
}

impl<K: Display + Ord, V: Display> BSTMap<K, V> {
    // TODO: Probably remove this/change to debug-only
    // TODO: clean this code up and change it into a BFS function?
    pub fn pretty_print(&self) -> () {
        let mut current_vector = vec![&self.head];

        let mut space_count = 35;

        while current_vector.iter().any(|node| node.is_some()) {
            print!(
                "{}",
                std::iter::repeat(" ").take(space_count).collect::<String>()
            );
            for node in &current_vector {
                if node.is_some() {
                    print!(
                        "{:>3}:{:>3}   ",
                        node.as_ref().unwrap().key,
                        node.as_ref().unwrap().value
                    );
                } else {
                    print!("X   ");
                }
            }
            println!("");

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
