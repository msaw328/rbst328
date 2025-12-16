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
    mem,
};

mod iter;
use crate::iter::{
    BSTMapByrefBreadthfirstIter, BSTMapByrefInorderIter, BSTMapByrefInorderIterMut,
    BSTMapConsumingInorderIter,
};

mod debug;

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
    length: usize,
}

impl<K: Ord, V> BSTMap<K, V> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.length = 0;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut current_node = &mut self.head;

        while current_node.is_some() {
            // unwrap is safe inside the loop, since current_node is Some
            let inner = current_node.as_mut().unwrap();

            current_node = match inner.key.cmp(&key) {
                Ordering::Less => &mut inner.right,
                Ordering::Greater => &mut inner.left,
                Ordering::Equal => {
                    // If we find node with equal key, means the value already exists - replace and early return
                    // safe unwrap - current_node is Some
                    let dest = &mut inner.as_mut().value;
                    return Some(mem::replace(dest, value));
                }
            }
        }

        // If we left the loop means value does not exist
        *current_node = Some(Box::new(Node::new(key, value)));
        self.length += 1;
        None
    }

    pub fn contains(&self, key: K) -> bool {
        let mut current_node = &self.head;

        while let Some(inner) = current_node.as_ref() {
            // unwrap is safe inside the loop, since current_node is Some
            current_node = match inner.key.cmp(&key) {
                Ordering::Less => &inner.right,
                Ordering::Greater => &inner.left,
                Ordering::Equal => return true,
            }
        }

        false
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut current_node = &self.head;

        while let Some(inner) = current_node.as_ref() {
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

        while let Some(inner) = current_node.as_mut() {
            current_node = match inner.key.cmp(&key) {
                Ordering::Less => &mut inner.right,
                Ordering::Greater => &mut inner.left,
                Ordering::Equal => return Some(&mut inner.value),
            }
        }

        None
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        // First - find current node, if it is even in there
        let mut current_node = &mut self.head;

        current_node = loop {
            if current_node.is_none() {
                return None;
            };

            // current_node is Some, so unwrap is safe
            current_node = match current_node.as_ref().unwrap().key.cmp(&key) {
                Ordering::Less => &mut current_node.as_mut().unwrap().right,
                Ordering::Greater => &mut current_node.as_mut().unwrap().left,
                Ordering::Equal => break current_node,
            }
        };

        // at this point we found a node, so something is getting removed, update length in advance
        self.length -= 1;

        let inner = current_node.as_ref().unwrap();

        // Below cases are from the wikipedia article: https://en.wikipedia.org/wiki/Binary_search_tree#Deletion
        // Case 1 - leaf node - just remove and call it a day
        if inner.right.is_none() && inner.left.is_none() {
            return Some(current_node.take().unwrap().value);
        }

        // Case 2 - one child - replace parent with child
        // At this point we are guaranteed that at least one of left/right is Some
        // (due to If above) so unwraps in two If's below are safe
        if inner.right.is_none() {
            let Node {
                left: saved_left,
                value: saved_value,
                ..
            } = *current_node.take().unwrap();

            *current_node = saved_left;

            return Some(saved_value);
        }

        if inner.left.is_none() {
            let Node {
                right: saved_right,
                value: saved_value,
                ..
            } = *current_node.take().unwrap();

            *current_node = saved_right;

            return Some(saved_value);
        }

        // Case 3 - two children - search for in order successor
        // Case 3a - if in order successor is immediately the right node (right node has no left subtree)
        // then replace parent with it, while keeping the left subtree
        // At this point both children are guaranteed to be Some so unwrap is safe
        if inner.right.as_ref().unwrap().left.is_none() {
            let Node {
                left: saved_left,
                right: saved_right,
                value: saved_value,
                ..
            } = *current_node.take().unwrap();

            *current_node = saved_right; // replace current node with right subtree of successor
            current_node.as_mut().unwrap().left = saved_left; // append saved left subtree

            return Some(saved_value);
        }

        // Case 3b - in order successor is not immediately the right node - search for it
        // need to keep reference to successors parent in order to replace successor
        // successor is the left child of successors parent
        // at the beginning we are guaranteed that left node exists due to earlier if(), so unwrap is safe

        // TODO: clean this code up, if possible
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

        // Take the current node, since it is being removed (save value for now to return it)
        let Node {
            left: saved_left,
            right: saved_right,
            value: saved_value,
            ..
        } = *current_node.take().unwrap();

        // Replace removed node with successor
        successor_inner.right = saved_right;
        successor_inner.left = saved_left;
        *current_node = Some(successor_inner);

        Some(saved_value)
    }

    pub fn iter_inorder(&self) -> BSTMapByrefInorderIter<'_, K, V> {
        BSTMapByrefInorderIter::new(self)
    }

    pub fn iter_inorder_mut(&mut self) -> BSTMapByrefInorderIterMut<'_, K, V> {
        BSTMapByrefInorderIterMut::new(self)
    }

    pub fn into_iter_inorder(self) -> BSTMapConsumingInorderIter<K, V> {
        BSTMapConsumingInorderIter::new(self)
    }

    pub fn iter_breadthfirst(&self) -> BSTMapByrefBreadthfirstIter<'_, K, V> {
        BSTMapByrefBreadthfirstIter::new(self)
    }

    pub fn iter(&self) -> BSTMapByrefInorderIter<'_, K, V> {
        self.iter_inorder()
    }

    pub fn iter_mut(&mut self) -> BSTMapByrefInorderIterMut<'_, K, V> {
        self.iter_inorder_mut()
    }
}

impl<K: Ord, V> Default for BSTMap<K, V> {
    fn default() -> Self {
        Self::new()
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

        bst.remove(KEY);

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());
    }

    #[test]
    fn retrieval_of_nonexistent_key_returns_none() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 999;

        assert!(!bst.contains(KEY));
        assert!(bst.get(KEY).is_none());
        assert!(bst.get_mut(KEY).is_none());
    }

    #[test]
    fn retrieval_of_existent_key_returns_some() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 999;
        const VALUE: &str = "something";
        bst.insert(KEY, VALUE.to_string());

        assert!(bst.contains(KEY));
        assert!(bst.get(KEY).is_some());
        assert!(bst.get_mut(KEY).is_some());
        assert_eq!(*bst.get(KEY).unwrap(), VALUE.to_string());
        assert_eq!(*bst.get_mut(KEY).unwrap(), VALUE.to_string());
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
        let return_val = bst.remove(KEY);

        assert!(return_val.is_none());
    }

    #[test]
    fn removal_returns_some_on_existent_key() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 1;
        const VALUE: &str = "hello";
        bst.insert(KEY, VALUE.to_string());

        let return_val = bst.remove(KEY);

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
        let mut return_val = bst.remove(5);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "hi".to_string());

        // child should remain accessible
        let mut child_node = bst.get(2);

        assert!(child_node.is_some());
        assert_eq!(*child_node.unwrap(), "leaf_node_child".to_string());

        // test removal of parent with right child
        return_val = bst.remove(15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 2);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // child should remain accessible
        assert!(bst.contains(20));
        child_node = bst.get(20);

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
        let return_val = bst.remove(15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // children should remain accessible
        for (k, v) in &CHILDREN_TO_CHECK {
            assert!(bst.contains(*k));
            let child_node = bst.get(*k);

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
        let return_val = bst.remove(15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // children should remain accessible
        for (k, v) in &CHILDREN_TO_CHECK {
            assert!(bst.contains(*k));
            let child_node = bst.get(*k);

            assert!(child_node.is_some());
            assert_eq!(*child_node.unwrap(), v.to_string());
        }
    }
}
