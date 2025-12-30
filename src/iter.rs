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

use std::collections::VecDeque;

use super::{BSTMap, Node};

// what parts of the node have been visited - nothing, left subtree, node itself, right subtree
// used by BSTMapByrefInorderIterator to add data about visited nodes to otherwise immutable tree
pub(crate) enum Visited {
    None,
    Left,
    Node,
    Right,
}

// Implements In-Order iteration over the BST
pub struct BSTMapByrefInorderIter<'a, K: Ord, V> {
    pub(crate) stack: Vec<(&'a Node<K, V>, Visited)>,
}

impl<'a, K: Ord, V> BSTMapByrefInorderIter<'a, K, V> {
    pub(crate) fn new(bst: &'a BSTMap<K, V>) -> Self {
        let stack = match &bst.head {
            None => Vec::new(),
            Some(inner_node) => {
                let mut s = Vec::with_capacity(bst.len());
                s.push((inner_node.as_ref(), Visited::None));
                s
            }
        };
        Self { stack }
    }
}

impl<'a, K: 'a + Ord, V: 'a> Iterator for BSTMapByrefInorderIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tuple) = self.stack.last_mut() {
            let (current_node, visited) = tuple;

            match *visited {
                Visited::None => {
                    *visited = Visited::Left;
                    if let Some(left_node) = &current_node.left {
                        self.stack.push((left_node.as_ref(), Visited::None));
                    }
                }

                Visited::Left => {
                    *visited = Visited::Node;
                    return Some((&current_node.key, &current_node.value));
                }

                Visited::Node => {
                    *visited = Visited::Right;
                    if let Some(right_node) = &current_node.right {
                        self.stack.push((right_node.as_ref(), Visited::None));
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

impl<'a, K: 'a + Ord, V: 'a> From<&'a BSTMap<K, V>> for BSTMapByrefInorderIter<'a, K, V> {
    fn from(value: &'a BSTMap<K, V>) -> Self {
        Self::new(value)
    }
}

// type aliases for BSTMapByrefInorderIterMut
// TODO: Replace with a proper struct maybe?
// OptionalSubtree - owned Option with &mut to a Node<K, V> - Some if unexplored, None if empty or explored
// OptionalKVMut - a tuple of shared reference to key type and mutable reference to value type
// LeftKVMutRight - a tuple containing, in order: optional left subtree to be explored, KV references, right subtree to be explored
type OptionalSubtree<'a, K, V> = Option<&'a mut Node<K, V>>;
type OptionalKVMut<'a, K, V> = Option<(&'a K, &'a mut V)>;
type LeftKVMutRight<'a, K, V> = (
    OptionalSubtree<'a, K, V>,
    OptionalKVMut<'a, K, V>,
    OptionalSubtree<'a, K, V>,
);
pub struct BSTMapByrefInorderIterMut<'a, K: 'a + Ord, V: 'a> {
    pub(crate) stack: Vec<LeftKVMutRight<'a, K, V>>,
}

impl<'a, K: 'a + Ord, V: 'a> BSTMapByrefInorderIterMut<'a, K, V> {
    pub(crate) fn new(bst: &'a mut BSTMap<K, V>) -> Self {
        let bst_len = bst.len();
        let stack = match &mut bst.head {
            None => Vec::new(),
            Some(inner_node) => {
                let mut s = Vec::with_capacity(bst_len);

                let Node {
                    left,
                    right,
                    key,
                    value,
                    ..
                } = inner_node.as_mut();

                s.push((
                    left.as_mut().map(|node| node.as_mut()),
                    Some((&*key, value)),
                    right.as_mut().map(|node| node.as_mut()),
                ));
                s
            }
        };
        Self { stack }
    }
}

impl<'a, K: 'a + Ord, V: 'a> Iterator for BSTMapByrefInorderIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(tuple) = self.stack.last_mut() {
            let (left_ref, current_kv, right_ref) = tuple;

            if let Some(inner_node) = left_ref.take() {
                let Node {
                    left,
                    right,
                    key,
                    value,
                    ..
                } = inner_node;

                self.stack.push((
                    left.as_mut().map(|node| node.as_mut()),
                    Some((&*key, value)),
                    right.as_mut().map(|node| node.as_mut()),
                ));

                continue;
            }

            if let Some(inner_node) = current_kv.take() {
                return Some(inner_node);
            }

            if let Some(inner_node) = right_ref.take() {
                let Node {
                    left,
                    right,
                    key,
                    value,
                    ..
                } = inner_node;

                self.stack.push((
                    left.as_mut().map(|node| node.as_mut()),
                    Some((&*key, value)),
                    right.as_mut().map(|node| node.as_mut()),
                ));

                continue;
            }

            self.stack.pop();
        }

        None
    }
}

impl<'a, K: 'a + Ord, V: 'a> From<&'a mut BSTMap<K, V>> for BSTMapByrefInorderIterMut<'a, K, V> {
    fn from(value: &'a mut BSTMap<K, V>) -> Self {
        Self::new(value)
    }
}

// Implements breadth-first iterator over BSTMap
pub struct BSTMapByrefBreadthfirstIter<'a, K: Ord, V> {
    pub(crate) queue: VecDeque<&'a Node<K, V>>,
}

impl<'a, K: Ord, V> BSTMapByrefBreadthfirstIter<'a, K, V> {
    pub(crate) fn new(bst: &'a BSTMap<K, V>) -> Self {
        let queue = match &bst.head {
            None => VecDeque::new(),
            Some(inner_node) => {
                let mut q = VecDeque::with_capacity(bst.len());
                q.push_back(inner_node.as_ref());
                q
            }
        };

        Self { queue }
    }
}

impl<'a, K: 'a + Ord, V: 'a> Iterator for BSTMapByrefBreadthfirstIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let next_element = self.queue.pop_front();

        // safe due to if
        let next_node = next_element?;

        if let Some(left_node) = &next_node.left {
            self.queue.push_back(left_node.as_ref());
        }

        if let Some(right_node) = &next_node.right {
            self.queue.push_back(right_node.as_ref());
        }

        Some((&next_node.key, &next_node.value))
    }
}

impl<'a, K: 'a + Ord, V: 'a> From<&'a BSTMap<K, V>> for BSTMapByrefBreadthfirstIter<'a, K, V> {
    fn from(value: &'a BSTMap<K, V>) -> Self {
        Self::new(value)
    }
}

pub struct BSTMapConsumingInorderIter<K, V> {
    pub(crate) stack: Vec<Box<Node<K, V>>>,
}

impl<K: Ord, V> BSTMapConsumingInorderIter<K, V> {
    pub(crate) fn new(mut bst: BSTMap<K, V>) -> Self {
        let bst_len = bst.len();
        let stack = match bst.head.take() {
            None => Vec::new(),
            Some(inner_node) => {
                let mut s = Vec::with_capacity(bst_len);
                s.push(inner_node);
                s
            }
        };

        Self { stack }
    }
}

impl<K: Ord, V> Iterator for BSTMapConsumingInorderIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current_node) = self.stack.last_mut() {
            let saved_left = current_node.left.take();

            // If left subtree exists
            if let Some(saved_left) = saved_left {
                self.stack.push(saved_left);
            } else {
                // If no left subtree, it means we want to consume the top of stack
                // unwrap safe since we're in while let Some on last_mut()
                let mut current_node = self.stack.pop().unwrap();

                let saved_right = current_node.right.take();

                // If right subtree exists push it to stack for further traversal
                if let Some(saved_right) = saved_right {
                    self.stack.push(saved_right);
                }

                return Some((current_node.key, current_node.value));
            }
        }

        None
    }
}

impl<K: Ord, V> From<BSTMap<K, V>> for BSTMapConsumingInorderIter<K, V> {
    fn from(value: BSTMap<K, V>) -> Self {
        Self::new(value)
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a BSTMap<K, V> {
    type Item = (&'a K, &'a V);

    type IntoIter = BSTMapByrefInorderIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_inorder()
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a mut BSTMap<K, V> {
    type Item = (&'a K, &'a mut V);

    type IntoIter = BSTMapByrefInorderIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_inorder_mut()
    }
}

impl<K: Ord, V> IntoIterator for BSTMap<K, V> {
    type Item = (K, V);

    type IntoIter = BSTMapConsumingInorderIter<K, V>;

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
    use crate::iter::{
        BSTMapByrefBreadthfirstIter, BSTMapByrefInorderIter, BSTMapByrefInorderIterMut,
        BSTMapConsumingInorderIter,
    };

    use super::BSTMap;

    // TODO: maybe deduplicate code for various iterators?

    #[test]
    fn byref_inorder_iter_is_empty_from_empty_map() {
        let bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = BSTMapByrefInorderIter::new(&bst);
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

        let collected: Vec<(&u32, &String)> = BSTMapByrefInorderIter::new(&bst).collect();

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

        let collected: Vec<_> = BSTMapByrefInorderIter::new(&bst).map(|(k, _)| *k).collect();

        assert!(collected.is_sorted());
    }

    #[test]
    fn byref_inorder_iter_mut_is_empty_from_empty_map() {
        let mut bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = BSTMapByrefInorderIterMut::new(&mut bst);
        let next_item = iter.next();

        assert!(next_item.is_none());
    }

    #[test]
    fn byref_inorder_iter_mut_contains_all_items() {
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

        let bst_len = bst.len();

        let collected: Vec<(&u32, &mut String)> =
            BSTMapByrefInorderIterMut::new(&mut bst).collect();

        const SERIES_OF_CHECKS: [(u32, &str); 4] =
            [(13, "hello"), (15, "bye"), (2, "test2"), (8, "high number")];

        assert_eq!(collected.len(), bst_len);

        for (k, v) in &SERIES_OF_CHECKS {
            assert!(
                collected
                    .iter()
                    .any(|(k_iter, v_iter)| *k == **k_iter && *v == **v_iter)
            );
        }
    }

    #[test]
    fn byref_inorder_iter_mut_is_sorted_by_key() {
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

        let collected: Vec<_> = BSTMapByrefInorderIterMut::new(&mut bst)
            .map(|(k, _)| *k)
            .collect();

        assert!(collected.is_sorted());
    }

    #[test]
    fn byref_inorder_iter_mut_modifies_all_entries() {
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

        let iter = BSTMapByrefInorderIterMut::new(&mut bst);

        for (_, v) in iter {
            v.insert_str(0, "AAA");
        }

        assert!(bst.iter().all(|(_, v)| v.starts_with("AAA")));
    }

    #[test]
    fn consuming_inorder_iter_is_empty_from_empty_map() {
        let bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = BSTMapConsumingInorderIter::new(bst);
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

        let collected: Vec<(u32, String)> = BSTMapConsumingInorderIter::new(bst).collect();

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

        let collected: Vec<_> = BSTMapConsumingInorderIter::new(bst)
            .map(|(k, _)| k)
            .collect();

        assert!(collected.is_sorted());
    }

    #[test]
    fn byref_breadthfirst_iter_is_empty_from_empty_map() {
        let bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = BSTMapByrefBreadthfirstIter::new(&bst);
        let next_item = iter.next();

        assert!(next_item.is_none());
    }

    #[test]
    fn byref_breadthfirst_iter_contains_all_items() {
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

        let collected: Vec<(&u32, &String)> = BSTMapByrefBreadthfirstIter::new(&bst).collect();

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
}
