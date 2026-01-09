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

//! Functionality related to the Map data structure based on an AVL Binary Search Tree.
//!
//! Main module exports the base data structure, while the `iter` module contains iterators.

pub mod iter;
use iter::*;

use std::{
    cmp::{Ord, Ordering},
    collections::VecDeque,
    fmt,
};

use crate::node::{NullableSubtreeAnchor, SubtreeAnchor};

/// Ordered Map based on a self-balancing AVL Binary Search Tree.
pub struct BSTMap<K: Ord, V> {
    /// Possibly null (None) reference to the root Node.
    pub(crate) head: NullableSubtreeAnchor<K, V>,
    /// Number of Nodes (elements) in the tree.
    length: usize,
}

impl<K: Ord, V> BSTMap<K, V> {
    /// Creates a new, empty BSTMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::map::BSTMap;
    ///
    /// // You most likely want to keep it mut to modify it
    /// let mut map: BSTMap<u32, String> = BSTMap::new();
    /// ```
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    /// Returns current length of the BSTMap (number of elements).
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if BSTMap is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Drops all data nodes from the BSTMap, making it empty afterwards.
    pub fn clear(&mut self) {
        self.drop_all_nodes_iteratively();
        self.length = 0;
    }

    /// Drops all nodes using a queue in an iterative manner.
    /// Used by clear() and drop() in order to avoid recursion.
    fn drop_all_nodes_iteratively(&mut self) {
        if self.head.is_none() {
            return;
        };

        let mut queue = VecDeque::<SubtreeAnchor<K, V>>::with_capacity(self.len());

        queue.push_front(self.head.take().unwrap());

        while let Some(mut node_box) = queue.pop_back() {
            if let Some(node_l) = node_box.left_mut().take() {
                queue.push_front(node_l);
            };

            if let Some(node_r) = node_box.right_mut().take() {
                queue.push_front(node_r);
            };

            drop(node_box);
        }

        self.length = 0;
    }

    /// Inserts a new value into the BSTMap, indexed by the given key.
    /// If the key already exists, the value is overwritten and no new data nodes are allocated.
    ///
    /// If the value was overwritten, old value will be returned by the call, encapsulated in `Some`.
    /// Otherwise, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::map::BSTMap;
    ///
    /// let mut map: BSTMap<u32, String> = BSTMap::new();
    /// map.insert(32, "Hello!".to_string());
    ///
    /// let old_value1 = map.insert(90, "World!".to_string());
    /// let old_value2 = map.insert(90, "World but different!".to_string());
    ///
    /// assert!(old_value1.is_none());
    /// assert!(old_value2.is_some());
    /// assert_eq!(*old_value2.unwrap(), "World!".to_string());
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        #[derive(PartialEq)]
        enum Subtree {
            Left,
            Right,
        }

        // TODO: consider preallocation of log(bst.len())
        // Maybe it is not necessary though, since insert wont probably always reach max depth
        let mut node_stack = Vec::new();
        let mut next_candidate = self.head.take();

        // loop until a node to replace is found (finds reference to Some that is to be replaced, or None that is to be filled)
        loop {
            // If next candidate SubtreeAnchor is None, it means this is where new value should be filled
            if next_candidate.is_none() {
                break;
            };

            // if next_candidate is equal to key, it means we're replacing it's value - no stack pushing needed
            // if subtree is to be explored, push current candidate node and subtree left/right info to stack
            let next_candidate_inner = next_candidate.as_mut().unwrap();
            let (next_candidate_replacement, subtree) = match next_candidate_inner.key().cmp(&key) {
                Ordering::Less => (next_candidate_inner.right_mut().take(), Subtree::Right),
                Ordering::Greater => (next_candidate_inner.left_mut().take(), Subtree::Left),
                Ordering::Equal => break,
            };

            // Push processed node on the stack
            node_stack.push((next_candidate.unwrap(), subtree));

            // Next candidate is either left or right subtree
            next_candidate = next_candidate_replacement;
        }

        // In the end, next_candidate was either None or Some and it is the node that is supposed to be replaced
        let mut node_to_be_replaced = next_candidate;

        // If the Node to be replaced is Some, replace it and dont change length
        // if the Node is None, insert a new node in its place and change length
        let return_value = if let Some(inner_node) = node_to_be_replaced.as_mut() {
            Some(inner_node.replace(value))
        } else {
            node_to_be_replaced = Some(SubtreeAnchor::new_leaf(key, value));
            self.length += 1;
            None
        };

        let mut child_node = node_to_be_replaced;

        // unwind the stack, rebuild the tree
        while let Some((mut parent_node, subtree)) = node_stack.pop() {
            match subtree {
                Subtree::Left => *parent_node.left_mut() = child_node,
                Subtree::Right => *parent_node.right_mut() = child_node,
            }

            // If we're not returning anything, it means a new value
            // was inserted - length changed
            if return_value.is_none() {
                parent_node.balance_subtree();
            }

            child_node = Some(parent_node);
        }

        self.head = child_node;

        return_value
    }

    /// Returns `true` if a given key exists in the BSTMap, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::map::BSTMap;
    ///
    /// let mut map: BSTMap<u32, String> = BSTMap::new();
    /// map.insert(32, "Hello!".to_string());
    ///
    /// assert!(map.contains(&32));
    /// assert!(!map.contains(&999));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        let mut current_node = &self.head;

        while let Some(inner) = current_node.as_ref() {
            // unwrap is safe inside the loop, since current_node is Some
            current_node = match inner.key().cmp(key) {
                Ordering::Less => inner.right(),
                Ordering::Greater => inner.left(),
                Ordering::Equal => return true,
            }
        }

        false
    }

    /// Returns a shared reference to the value associated with given key inside the BSTMap, encapsulated by `Some`.
    /// If the BSTMap does not contain given key, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::map::BSTMap;
    ///
    /// let mut map: BSTMap<u32, String> = BSTMap::new();
    /// map.insert(32, "Hello!".to_string());
    ///
    /// let nonexistent = map.get(&999);
    /// let existent = map.get(&32);
    ///
    /// assert!(existent.is_some());
    /// assert_eq!(*existent.unwrap(), "Hello!".to_string());
    ///
    /// assert!(nonexistent.is_none());
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut current_node = &self.head;

        while let Some(inner) = current_node.as_ref() {
            current_node = match inner.key().cmp(key) {
                Ordering::Less => inner.right(),
                Ordering::Greater => inner.left(),
                Ordering::Equal => return Some(inner.value()),
            }
        }

        None
    }
    /// Returns a mutable reference to the value associated with given key inside the BSTMap, encapsulated by `Some`.
    /// If the BSTMap does not contain given key, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::map::BSTMap;
    ///
    /// let mut map: BSTMap<u32, String> = BSTMap::new();
    /// map.insert(32, "Hello!".to_string());
    ///
    /// let nonexistent = map.get_mut(&999);
    /// assert!(nonexistent.is_none());
    ///
    /// let existent = map.get_mut(&32);
    /// assert!(existent.is_some());
    ///
    /// let val_reference = existent.unwrap();
    /// val_reference.insert(0, 'T');
    ///
    /// assert_eq!(*map.get(&32).unwrap(), "THello!".to_string());
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut current_node = &mut self.head;

        while let Some(inner) = current_node.as_mut() {
            current_node = match inner.key().cmp(key) {
                Ordering::Less => inner.right_mut(),
                Ordering::Greater => inner.left_mut(),
                Ordering::Equal => return Some(inner.value_mut()),
            }
        }

        None
    }
    /// Removes the value at the given key from the BSTMap and returns it encapsulated in `Some`.
    /// If given key does not exist in the BSTMap, the method does nothing and returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::map::BSTMap;
    ///
    /// let mut map: BSTMap<u32, String> = BSTMap::new();
    /// map.insert(32, "Hello!".to_string());
    ///
    /// let old_value1 = map.remove(&90);
    /// let old_value2 = map.remove(&32);
    /// let old_value3 = map.remove(&32);
    ///
    /// assert!(old_value1.is_none());
    /// assert!(old_value2.is_some());
    /// assert_eq!(old_value2.unwrap(), "Hello!".to_string());
    /// assert!(old_value3.is_none());
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        #[derive(PartialEq)]
        enum Subtree {
            Left,
            Right,
        }

        // TODO: consider preallocation of log(bst.len())
        // Maybe it is not necessary though, since insert wont probably always reach max depth
        let mut node_stack = Vec::new();
        let mut next_candidate = self.head.take();

        // loop until a node to remove is found (finds reference to Some that is to be replaced, or None that is to be filled)
        loop {
            // If next candidate NodeRef is None, it means this is where new value should be filled
            if next_candidate.is_none() {
                break;
            };

            // if next_candidate is equal to key, it means we're replacing it's value - no stack pushing needed
            // if subtree is to be explored, push current candidate node and subtree left/right info to stack
            let next_candidate_inner = next_candidate.as_mut().unwrap();
            let (next_candidate_replacement, subtree) = match next_candidate_inner.key().cmp(key) {
                Ordering::Less => (next_candidate_inner.right_mut().take(), Subtree::Right),
                Ordering::Greater => (next_candidate_inner.left_mut().take(), Subtree::Left),
                Ordering::Equal => break,
            };

            // Push processed node on the stack
            node_stack.push((next_candidate.unwrap(), subtree));

            // Next candidate is either left or right subtree
            next_candidate = next_candidate_replacement;
        }

        // In the end, next_candidate was either None or Some and it is the node that is supposed to be replaced
        let node_to_be_removed = next_candidate;

        // If found node is Some, destroy it and store returned value
        // else, reinsert itself to the tree and return None
        let (mut child_node, return_value) = if let Some(mut inner_node) = node_to_be_removed {
            let replacement_tree = inner_node.remove();
            self.length -= 1;
            (replacement_tree, Some(inner_node.consume_kv().1))
        } else {
            (node_to_be_removed, None)
        };

        // unwind the stack, rebuild the tree
        while let Some((mut parent_node, subtree)) = node_stack.pop() {
            match subtree {
                Subtree::Left => *parent_node.left_mut() = child_node,
                Subtree::Right => *parent_node.right_mut() = child_node,
            }

            // If we're returning something, it means a value
            // was removed - length changed
            if return_value.is_some() {
                parent_node.balance_subtree();
            }

            child_node = Some(parent_node);
        }

        self.head = child_node;

        return_value
    }

    /// Returns an iterator which iterates by all the key-value pairs breadth-first.
    /// It yields a shared reference to each key and value assigned to it.
    pub fn iter_bfs(&self) -> BFSIter<'_, K, V> {
        BFSIter::new(self)
    }

    /// Returns the default in order keys iterator.
    ///
    /// Yields shared reference to each key.
    pub fn keys(&self) -> KeysIter<'_, K, V> {
        KeysIter::new(self)
    }

    /// Consumes the map and returns an owned iterator over the keys.
    ///
    /// Yields each key in order.
    pub fn into_keys(self) -> KeysIntoIter<K, V> {
        KeysIntoIter::new(self)
    }

    /// Returns the default values iterator.
    pub fn values(&self) -> ValuesIter<'_, K, V> {
        ValuesIter::new(self)
    }

    /// Returns the default mutable values iterator.
    pub fn values_mut(&mut self) -> ValuesIterMut<'_, K, V> {
        ValuesIterMut::new(self)
    }

    /// Consumes the map and returns an owned iterator over the values.
    ///
    /// Yields each value in order of keys.
    pub fn into_values(self) -> ValuesIntoIter<K, V> {
        ValuesIntoIter::new(self)
    }

    /// Returns the default key-value pair iterator.
    pub fn iter(&self) -> InorderIter<'_, K, V> {
        InorderIter::new(self)
    }

    /// Returns the default mutable key-value pair iterator.
    pub fn iter_mut(&mut self) -> InorderIterMut<'_, K, V> {
        InorderIterMut::new(self)
    }
}

/// By default, create an empty BSTMap.
impl<K: Ord, V> Default for BSTMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: fmt::Debug + Ord, V: fmt::Debug> fmt::Debug for BSTMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// In order to avoid recursive calls to drop provide an iterative version.
impl<K: Ord, V> Drop for BSTMap<K, V> {
    fn drop(&mut self) {
        self.drop_all_nodes_iteratively();
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
    fn new_map_is_empty() {
        let bst = BSTMap::<u32, String>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn insertion_changes_length_but_duplicates_do_not() {
        let mut bst = BSTMap::<u32, String>::new();

        const SERIES_OF_INSERTIONS: [(u32, &str); 6] = [
            (12, "hello"),
            (35, "bye"),
            (999, "test"),
            (0, "test2"),
            (1200, "high number"),
            (36, "utf8 string ąąąąą"),
        ];
        const SERIES_OF_DUPLICATES: [(u32, &str); 6] = [
            (12, "hello1"),
            (35, "bye2"),
            (999, "test3"),
            (0, "test24"),
            (1200, "high number5"),
            (36, "utf8 string ąąąąą6"),
        ];

        for (k, v) in &SERIES_OF_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        for (k, v) in &SERIES_OF_DUPLICATES {
            bst.insert(*k, v.to_string());
        }

        // Duplicates should not change length!
        assert_eq!(bst.len(), SERIES_OF_INSERTIONS.len());
        assert!(!bst.is_empty());
    }

    #[test]
    fn removal_changes_length() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 1;
        const VALUE: &str = "test";
        bst.insert(KEY, VALUE.to_string());

        bst.remove(&KEY);

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn retrieval_of_nonexistent_key_returns_none() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 999;

        assert!(!bst.contains(&KEY));
        assert!(bst.get(&KEY).is_none());
        assert!(bst.get_mut(&KEY).is_none());
    }

    #[test]
    fn retrieval_of_existent_key_returns_some() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 999;
        const VALUE: &str = "something";
        bst.insert(KEY, VALUE.to_string());

        assert!(bst.contains(&KEY));
        assert!(bst.get(&KEY).is_some());
        assert!(bst.get_mut(&KEY).is_some());
        assert_eq!(*bst.get(&KEY).unwrap(), VALUE.to_string());
        assert_eq!(*bst.get_mut(&KEY).unwrap(), VALUE.to_string());
    }

    #[test]
    fn insertion_overwrites_existing_key() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 999;
        const ORIGINAL_VALUE: &str = "hallo";
        const NEW_VALUE: &str = "bye";
        let mut return_val = bst.insert(KEY, ORIGINAL_VALUE.to_string());

        assert!(return_val.is_none());

        // overwrite
        return_val = bst.insert(KEY, NEW_VALUE.to_string());

        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), ORIGINAL_VALUE.to_string());
    }

    #[test]
    fn removal_returns_none_on_nonexistent_key() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 1;
        let return_val = bst.remove(&KEY);

        assert!(return_val.is_none());
    }

    #[test]
    fn removal_returns_some_on_existent_key() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 1;
        const VALUE: &str = "hello";
        bst.insert(KEY, VALUE.to_string());

        let return_val = bst.remove(&KEY);

        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), VALUE.to_string());
    }

    #[test]
    fn clear_sets_length_to_zero() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 1;
        const VALUE: &str = "hi";
        bst.insert(KEY, VALUE.to_string());

        bst.clear();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn removal_of_node_with_one_child() {
        let mut bst = BSTMap::<u32, String>::new();

        const TEST_INSERTIONS: [(u32, &str); 5] = [
            (10, "hello"),
            (5, "hi"),
            (15, "bye"),
            (2, "leaf_node_child"),
            (20, "right_child"),
        ];
        for (k, v) in &TEST_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        //      10
        //     /  \
        //    5   15
        //   /      \
        //  2       20
        // test removal of parent with left child
        let mut return_val = bst.remove(&5);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "hi".to_string());

        // child should remain accessible
        let mut child_node = bst.get(&2);

        assert!(child_node.is_some());
        assert_eq!(*child_node.unwrap(), "leaf_node_child".to_string());

        // test removal of parent with right child
        return_val = bst.remove(&15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 2);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // child should remain accessible
        assert!(bst.contains(&20));
        child_node = bst.get(&20);

        assert!(child_node.is_some());
        assert_eq!(*child_node.unwrap(), "right_child".to_string());
    }

    #[test]
    fn removal_of_node_with_two_children_and_right_node_successor() {
        let mut bst = BSTMap::<u32, String>::new();

        const TEST_INSERTIONS: [(u32, &str); 8] = [
            (10, "hello"),
            (5, "hi"),
            (15, "bye"),
            (2, "leaf_node_child"),
            (13, "left_child"),
            (20, "right_child"),
            (12, "left_child_subtree_l"),
            (14, "left_child_subtree_r"),
        ];

        // Children to check after removing 15
        const CHILDREN_TO_CHECK: [(u32, &str); 4] = [
            (13, "left_child"),
            (20, "right_child"),
            (12, "left_child_subtree_l"),
            (14, "left_child_subtree_r"),
        ];
        for (k, v) in &TEST_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        //      10
        //     /  \
        //    5   15
        //   /   /  \
        //  2   13  20
        //     /  \
        //    12  14
        let return_val = bst.remove(&15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // children should remain accessible
        for (k, v) in &CHILDREN_TO_CHECK {
            assert!(bst.contains(k));
            let child_node = bst.get(k);

            assert!(child_node.is_some());
            assert_eq!(*child_node.unwrap(), v.to_string());
        }
    }

    #[test]
    fn removal_of_node_with_two_children_and_successor_in_right_subtree() {
        let mut bst = BSTMap::<u32, String>::new();

        const TEST_INSERTIONS: [(u32, &str); 11] = [
            (10, "hello"),
            (5, "hi"),
            (15, "bye"),
            (2, "leaf_node_child"),
            (13, "left_child"),
            (20, "right_child"),
            (12, "left_child_subtree_l"),
            (14, "left_child_subtree_r"),
            (19, "right_child_subtree_l"),
            (17, "right_child_subtree_l_l"),
            (21, "right_child_subtree_r"),
        ];

        // Children to check after removing 15
        const CHILDREN_TO_CHECK: [(u32, &str); 7] = [
            (13, "left_child"),
            (20, "right_child"),
            (12, "left_child_subtree_l"),
            (14, "left_child_subtree_r"),
            (19, "right_child_subtree_l"),
            (17, "right_child_subtree_l_l"),
            (21, "right_child_subtree_r"),
        ];
        for (k, v) in &TEST_INSERTIONS {
            bst.insert(*k, v.to_string());
        }

        //      10
        //     /  \
        //    5   15
        //   /   /  \
        //  2   13  20
        //     / |  | \
        //    12 14 19 21
        //         /
        //        17
        let return_val = bst.remove(&15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // children should remain accessible
        for (k, v) in &CHILDREN_TO_CHECK {
            assert!(bst.contains(k));
            let child_node = bst.get(k);

            assert!(child_node.is_some());
            assert_eq!(*child_node.unwrap(), v.to_string());
        }
    }
}
