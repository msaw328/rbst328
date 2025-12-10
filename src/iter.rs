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

use super::{BSTMap, NodeRef};

// what parts of the node have been visited - nothing, left subtree, node itself, right subtree
pub(crate) enum Visited {
    None,
    Left,
    Node,
    Right,
}
// Implements In-Order iteration over the BST
pub struct BSTMapIter<'a, K: Ord, V> {
    pub(crate) stack: Vec<(&'a NodeRef<K, V>, Visited)>,
}

impl<'a, K: Ord, V> BSTMapIter<'a, K, V> {
    pub(crate) fn new(bst: &'a BSTMap<K, V>) -> Self {
        Self {
            stack: match &bst.head {
                None => Vec::new(),
                Some(_) => Vec::from([(&bst.head, Visited::None)]),
            },
        }
    }
}

impl<'a, K: 'a + Ord, V: 'a> Iterator for BSTMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tuple) = self.stack.last_mut() {
            let (current_node, visited) = tuple;

            if current_node.is_none() {
                self.stack.pop();
                continue;
            }

            let inner_node = current_node.as_ref().unwrap();

            match *visited {
                Visited::None => {
                    *visited = Visited::Left;
                    if inner_node.left.is_some() {
                        self.stack.push((&inner_node.left, Visited::None));
                    }
                }

                Visited::Left => {
                    *visited = Visited::Node;
                    return Some((&inner_node.key, &inner_node.value));
                }

                Visited::Node => {
                    *visited = Visited::Right;
                    if inner_node.right.is_some() {
                        self.stack.push((&inner_node.right, Visited::None));
                    }
                }

                // Visited::Right - remove the node from stack
                Visited::Right => {
                    self.stack.pop();
                }
            }
        }

        None
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a BSTMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter = BSTMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct BSTMapIntoIter<K, V> {
    pub(crate) stack: Vec<(NodeRef<K, V>, Visited)>,
}

impl<K: Ord, V> BSTMapIntoIter<K, V> {
    pub(crate) fn new(bst: BSTMap<K, V>) -> Self {
        Self {
            stack: match &bst.head {
                None => Vec::new(),
                Some(_) => Vec::from([(bst.head, Visited::None)]),
            },
        }
    }
}

impl<K: Ord, V> Iterator for BSTMapIntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tuple) = self.stack.pop() {
            let (current_node, mut visited) = tuple;

            if current_node.is_none() {
                continue;
            }

            let mut inner_node = current_node.unwrap();

            match visited {
                Visited::None => {
                    visited = Visited::Left;

                    let saved_left = inner_node.left.take();

                    self.stack.push((Some(inner_node), visited));

                    if saved_left.is_some() {
                        self.stack.push((saved_left, Visited::None));
                    }
                }

                Visited::Left => {
                    let saved_right = inner_node.right.take();
                    self.stack.push((saved_right, Visited::None));
                    return Some((inner_node.key, inner_node.value));
                }

                _ => {}
            }
        }

        None
    }
}

impl<K: Ord, V> IntoIterator for BSTMap<K, V> {
    type Item = (K, V);

    type IntoIter = BSTMapIntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        BSTMapIntoIter::new(self)
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for BSTMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut bst = Self::new();

        for (k, v) in iter {
            bst.insert(k, v);
        }

        bst
    }
}

impl<K: Ord, V, const N: usize> From<[(K, V); N]> for BSTMap<K, V> {
    fn from(array: [(K, V); N]) -> Self {
        let mut bst = Self::new();

        for (k, v) in array {
            bst.insert(k, v);
        }

        bst
    }
}

impl<K: Ord, V> Extend<(K, V)> for BSTMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

// TODO: write unit tests for iterator stuff
