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
pub struct BSTMapByrefInorderIter<'a, K: Ord, V> {
    pub(crate) stack: Vec<(&'a NodeRef<K, V>, Visited)>,
}

impl<'a, K: Ord, V> BSTMapByrefInorderIter<'a, K, V> {
    pub(crate) fn new(bst: &'a BSTMap<K, V>) -> Self {
        Self {
            stack: match &bst.head {
                None => Vec::new(),
                Some(_) => Vec::from([(&bst.head, Visited::None)]),
            },
        }
    }
}

impl<'a, K: 'a + Ord, V: 'a> Iterator for BSTMapByrefInorderIter<'a, K, V> {
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

    type IntoIter = BSTMapByrefInorderIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_inorder()
    }
}

pub struct BSTMapIntoConsumingInorderIter<K, V> {
    pub(crate) stack: Vec<(NodeRef<K, V>, Visited)>,
}

impl<K: Ord, V> BSTMapIntoConsumingInorderIter<K, V> {
    pub(crate) fn new(bst: BSTMap<K, V>) -> Self {
        Self {
            stack: match &bst.head {
                None => Vec::new(),
                Some(_) => Vec::from([(bst.head, Visited::None)]),
            },
        }
    }
}

impl<K: Ord, V> Iterator for BSTMapIntoConsumingInorderIter<K, V> {
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

    type IntoIter = BSTMapIntoConsumingInorderIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter_inorder()
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

#[cfg(test)]
mod tests {
    use super::BSTMap;

    #[test]
    fn byref_inorder_iter_is_empty_from_empty_map() {
        let bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = bst.iter_inorder();
        let next_item = iter.next();

        assert!(next_item.is_none());
    }

    #[test]
    fn byref_inorder_iter_contains_all_items() {
        let mut bst = BSTMap::<u32, String>::new();

        const SERIES_OF_INSERTIONS: [(u32, &str); 5] = [
            (13, "hello"),
            (15, "bye"),
            (7, "test"),
            (2, "test2"),
            (8, "high number"),
        ];

        for (k, v) in &SERIES_OF_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        bst.remove(7); // remove non-leaf node

        let collected: Vec<(&u32, &String)> = bst.iter_inorder().collect();

        const SERIES_OF_CHECKS: [(u32, &str); 4] =
            [(13, "hello"), (15, "bye"), (2, "test2"), (8, "high number")];

        assert_eq!(collected.len(), bst.len());

        for (k, v) in &SERIES_OF_CHECKS {
            assert!(
                collected
                    .iter()
                    .any(|(k_iter, v_iter)| *k == **k_iter && *v == **v_iter)
            );
        }
    }

    #[test]
    fn byref_inorder_iter_is_sorted_by_key() {
        let mut bst = BSTMap::<u32, String>::new();

        const SERIES_OF_INSERTIONS: [(u32, &str); 5] = [
            (13, "hello"),
            (15, "bye"),
            (7, "test"),
            (2, "test2"),
            (8, "high number"),
        ];

        for (k, v) in &SERIES_OF_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        let collected: Vec<_> = bst.iter_inorder().map(|(k, _)| *k).collect();

        assert!(collected.is_sorted());
    }

    #[test]
    fn consuming_inorder_iter_is_empty_from_empty_map() {
        let bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = bst.into_iter();
        let next_item = iter.next();

        assert!(next_item.is_none());
    }

    #[test]
    fn consuming_inorder_iter_contains_all_items() {
        let mut bst = BSTMap::<u32, String>::new();

        const SERIES_OF_INSERTIONS: [(u32, &str); 5] = [
            (13, "hello"),
            (15, "bye"),
            (7, "test"),
            (2, "test2"),
            (8, "high number"),
        ];

        for (k, v) in &SERIES_OF_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        bst.remove(7); // remove non-leaf node

        let saved_len = bst.len();

        let collected: Vec<(u32, String)> = bst.into_iter().collect();

        const SERIES_OF_CHECKS: [(u32, &str); 4] =
            [(13, "hello"), (15, "bye"), (2, "test2"), (8, "high number")];

        assert_eq!(collected.len(), saved_len);

        for (k, v) in &SERIES_OF_CHECKS {
            assert!(
                collected
                    .iter()
                    .any(|(k_iter, v_iter)| *k == *k_iter && *v == *v_iter)
            );
        }
    }

    #[test]
    fn consuming_inorder_iter_is_sorted_by_key() {
        let mut bst = BSTMap::<u32, String>::new();

        const SERIES_OF_INSERTIONS: [(u32, &str); 5] = [
            (13, "hello"),
            (15, "bye"),
            (7, "test"),
            (2, "test2"),
            (8, "high number"),
        ];

        for (k, v) in &SERIES_OF_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        let collected: Vec<_> = bst.into_iter().map(|(k, _)| k).collect();

        assert!(collected.is_sorted());
    }
}
