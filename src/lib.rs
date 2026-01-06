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
    collections::VecDeque,
    mem,
};

mod iter;
use crate::{
    iter::{
        BSTMapByrefBreadthfirstIter, BSTMapByrefInorderIter, BSTMapByrefInorderIterMut,
        BSTMapConsumingInorderIter,
    },
    node::NodeRef,
};

mod debug;

mod node;
use crate::node::NullableNodeRef;

/*
// Shorthand for a referece to a Box'ed node that may or may not be there
type NodeRef<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> {
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
    height: i32,
    key: K,
    value: V,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            left: None,
            right: None,
            height: 1,
            value,
            key,
        }
    }

    pub fn left_height(&self) -> i32 {
        match &self.left {
            Some(node) => node.height,
            None => 0,
        }
    }

    pub fn right_height(&self) -> i32 {
        match &self.right {
            Some(node) => node.height,
            None => 0,
        }
    }

    pub fn balance(&self) -> i32 {
        self.right_height() - self.left_height()
    }

    pub fn update_height(&mut self) {
        self.height = 1 + self.left_height().max(self.right_height())
    }
}*/

pub struct BSTMap<K: Ord, V> {
    head: NullableNodeRef<K, V>,
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

    pub fn insert(&mut self, key_insert: K, value_insert: V) -> Option<V> {
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
            // If next candidate NodeRef is None, it means this is where new value should be filled
            if next_candidate.is_none() {
                break;
            };

            // if next_candidate is equal to key, it means we're replacing it's value - no stack pushing needed
            // if subtree is to be explored, push current candidate node and subtree left/right info to stack
            let next_candidate_inner = next_candidate.as_mut().unwrap();
            let (next_candidate_replacement, subtree) =
                match next_candidate_inner.key().cmp(&key_insert) {
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
        let node_to_be_replaced = next_candidate;

        // If the Node to be replaced is Some, it means there is no new node added
        // replace only the value and reinsert all the nodes back in the tree
        if let Some(mut inner_node) = node_to_be_replaced {
            let dest = inner_node.value_mut();
            let old_value = mem::replace(dest, value_insert);

            while let Some((mut parent_node, subtree)) = node_stack.pop() {
                match subtree {
                    Subtree::Left => *parent_node.left_mut() = Some(inner_node),
                    Subtree::Right => *parent_node.right_mut() = Some(inner_node),
                }

                inner_node = parent_node;
            }

            self.head = Some(inner_node);

            return Some(old_value);
        }

        // node to be replaced is None, so a new Node will be inserted
        // this requires us to fix all the ancestors in terms of balancing
        let mut inner_node = NodeRef::new(key_insert, value_insert);
        self.length += 1;

        while let Some((mut parent_node, subtree)) = node_stack.pop() {
            match subtree {
                Subtree::Left => *parent_node.left_mut() = Some(inner_node),
                Subtree::Right => *parent_node.right_mut() = Some(inner_node),
            }

            parent_node.balance_subtree();
            inner_node = parent_node;
        }

        self.head = Some(inner_node);

        // Since new node was inserted, return None for old_value
        None
    }

    pub fn contains(&self, key: K) -> bool {
        let mut current_node = &self.head;

        while let Some(inner) = current_node.as_ref() {
            // unwrap is safe inside the loop, since current_node is Some
            current_node = match inner.key().cmp(&key) {
                Ordering::Less => inner.right(),
                Ordering::Greater => inner.left(),
                Ordering::Equal => return true,
            }
        }

        false
    }

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

    pub fn remove(&mut self, key: K) -> Option<V> {
        // First - find current node, if it is even in there
        let mut current_node = &mut self.head;

        current_node = loop {
            if current_node.is_none() {
                return None;
            };

            // current_node is Some, so unwrap is safe
            current_node = match current_node.as_ref().unwrap().key().cmp(&key) {
                Ordering::Less => current_node.as_mut().unwrap().right_mut(),
                Ordering::Greater => current_node.as_mut().unwrap().left_mut(),
                Ordering::Equal => break current_node,
            }
        };

        // at this point we found a node, so something is getting removed, update length in advance
        self.length -= 1;

        let inner = current_node.as_ref().unwrap();

        // Below cases are from the wikipedia article: https://en.wikipedia.org/wiki/Binary_search_tree#Deletion
        // Case 1 - leaf node - just remove and call it a day
        if inner.right().is_none() && inner.left().is_none() {
            let node_ref = current_node.take().unwrap();
            return Some(node_ref.consume().value);
        }

        // Case 2 - one child - replace parent with child
        // At this point we are guaranteed that at least one of left/right is Some
        // (due to If above) so unwraps in two If's below are safe
        if inner.right().is_none() {
            let mut node_ref = current_node.take().unwrap();

            *current_node = node_ref.left_mut().take();

            return Some(node_ref.consume().value);
        }

        if inner.left().is_none() {
            let mut node_ref = current_node.take().unwrap();

            *current_node = node_ref.right_mut().take();

            return Some(node_ref.consume().value);
        }

        // Case 3a and 3b
        // If node is left-heavy, replace with predecessor
        // Otherwise, replace with successor

        let inner = current_node.as_mut().unwrap();

        let old_node = if inner.balance() < 0 {
            inner.replace_with_subtree_predecessor()
        } else {
            inner.replace_with_subtree_successor()
        };

        Some(old_node.consume().value)
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

// In order to avoid recursive calls to drop
// provide an iterative version
impl<K: Ord, V> Drop for BSTMap<K, V> {
    fn drop(&mut self) {
        if self.head.is_none() {
            return;
        };

        let mut queue = VecDeque::<NodeRef<K, V>>::with_capacity(self.len());

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
        assert!(bst.get(&KEY).is_none());
        assert!(bst.get_mut(&KEY).is_none());
    }

    #[test]
    fn retrieval_of_existent_key_returns_some() {
        let mut bst = BSTMap::<u32, String>::new();

        const KEY: u32 = 999;
        const VALUE: &str = "something";
        bst.insert(KEY, VALUE.to_string());

        assert!(bst.contains(KEY));
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
        let mut child_node = bst.get(&2);

        assert!(child_node.is_some());
        assert_eq!(*child_node.unwrap(), "leaf_node_child".to_string());

        // test removal of parent with right child
        return_val = bst.remove(15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 2);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // child should remain accessible
        assert!(bst.contains(20));
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
        let return_val = bst.remove(15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // children should remain accessible
        for (k, v) in &CHILDREN_TO_CHECK {
            assert!(bst.contains(*k));
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
        let return_val = bst.remove(15);

        assert_eq!(bst.len(), TEST_INSERTIONS.len() - 1);
        assert!(return_val.is_some());
        assert_eq!(return_val.unwrap(), "bye".to_string());

        // children should remain accessible
        for (k, v) in &CHILDREN_TO_CHECK {
            assert!(bst.contains(*k));
            let child_node = bst.get(k);

            assert!(child_node.is_some());
            assert_eq!(*child_node.unwrap(), v.to_string());
        }
    }
}
