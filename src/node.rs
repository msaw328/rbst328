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

use std::fmt::Display;

/// Data contained by the node.
///
/// This is a plain data structure, that is meant to be Box'ed and handled via a SubtreeAnchor.
#[derive(Debug)]
struct NodeData<K, V> {
    pub left: NullableSubtreeAnchor<K, V>,
    pub right: NullableSubtreeAnchor<K, V>,
    pub height: i32,
    pub key: K,
    pub value: V,
}

/// An anchor point for a subtree, which may be None.
pub type NullableSubtreeAnchor<K, V> = Option<SubtreeAnchor<K, V>>;

/// An anchor point for a Subtree (which may be a leaf node with no children).
///
/// Contains a Box reference to the NodeData for the root node of the subtree.
#[derive(Debug)]
pub(crate) struct SubtreeAnchor<K, V>(Box<NodeData<K, V>>);

impl<K, V> SubtreeAnchor<K, V> {
    /// Creates a new SubtreeAnchor as a single leaf node.
    pub fn new_leaf(key: K, value: V) -> Self {
        Self(Box::new(NodeData {
            left: None,
            right: None,
            height: 1,
            key,
            value,
        }))
    }

    /// Returns a shared reference to the left NullableSubtreeAnchor.
    pub fn left(&self) -> &NullableSubtreeAnchor<K, V> {
        &self.0.left
    }

    /// Returns a shared reference to the right NullableSubtreeAnchor.
    pub fn right(&self) -> &NullableSubtreeAnchor<K, V> {
        &self.0.right
    }

    /// Returns a mutable reference to the left NullableSubtreeAnchor.
    pub fn left_mut(&mut self) -> &mut NullableSubtreeAnchor<K, V> {
        &mut self.0.left
    }

    /// Returns a mutable reference to the right NullableSubtreeAnchor.
    pub fn right_mut(&mut self) -> &mut NullableSubtreeAnchor<K, V> {
        &mut self.0.right
    }

    /// Returns a shared reference to the key.
    ///
    /// There is no function returning a mutable reference to the key.
    /// Such a function would pose danger to validity of the structure of the tree.
    pub fn key(&self) -> &K {
        &self.0.key
    }

    /// Returns a shared reference to the value.
    pub fn value(&self) -> &V {
        &self.0.value
    }

    /// Returns a mutable reference to the value.
    pub fn value_mut(&mut self) -> &mut V {
        &mut self.0.value
    }

    /// Returns a tuple of shared references to key and value.
    pub fn kv(&self) -> (&K, &V) {
        (&self.0.key, &self.0.value)
    }

    /// Returns a tuple of mutable references to left, right and value plus a shared reference to key.
    pub fn split_mut(
        &mut self,
    ) -> (
        &mut NullableSubtreeAnchor<K, V>,
        &mut NullableSubtreeAnchor<K, V>,
        &K,
        &mut V,
    ) {
        let NodeData {
            left,
            right,
            key,
            value,
            ..
        } = self.0.as_mut();

        (left, right, key, value)
    }

    /// Consumes self to return NodeData contained within.
    /// Used for removal.
    fn consume(self) -> NodeData<K, V> {
        *self.0
    }

    /// Consumes self to return (K, V).
    pub fn consume_kv(self) -> (K, V) {
        let data = self.consume();
        (data.key, data.value)
    }

    /// Helper method used during removal of a Node.
    ///
    /// It mutates self, detaching the Left and Right subtrees from the Root.
    /// Afterwards, it modifies both subtrees in accordance with AVL BST rules to create the replacement subtree.
    /// It returns a NullableNodeRef which should replace this subtree in the larger BST structure.
    /// Returned subtree may be None if nothing should replace the removed subtree (e.g. when a leaf node is removed).
    pub fn remove(&mut self) -> NullableSubtreeAnchor<K, V> {
        // Case 1. Leaf node - just remove
        if self.left().is_none() && self.right().is_none() {
            return None;
        }

        // at this point we're guaranteed that at least one child exists

        // Case 2. One child - replace with the child that is not None
        if self.right().is_none() {
            return self.left_mut().take();
        }

        if self.left().is_none() {
            return self.right_mut().take();
        }

        // Case 3a and b - replace the child with successor or predecessor
        // replace with node from heavier subtree, so it becomes more balanced
        // instead of less
        Some(if self.balance() < 0 {
            self.replace_with_subtree_predecessor()
        } else {
            self.replace_with_subtree_successor()
        })
    }

    /// Returns AVL height of the left subtree.
    fn left_height(&self) -> i32 {
        match &self.0.left {
            Some(node) => node.0.height,
            None => 0,
        }
    }

    /// Returns AVL height of the right subtree.
    fn right_height(&self) -> i32 {
        match &self.0.right {
            Some(node) => node.0.height,
            None => 0,
        }
    }

    /// Returns AVL balance factor for the root of the subtree.
    pub fn balance(&self) -> i32 {
        self.right_height() - self.left_height()
    }

    /// Updates height of this node based on it's children heights.
    /// Soundness assumption: Both children have correct heights/are empty.
    fn update_height(&mut self) {
        self.0.height = 1 + self.left_height().max(self.right_height())
    }

    /// Performs an AVL single rotation to the right.
    /// Soundness assumption: self.left() is not None.
    fn rotate_right(&mut self) {
        let left_child_ref = self.left_mut();

        if left_child_ref.is_none() {
            panic!("AVL right rotation attempted on a Node with no left child!");
        }

        let mut left_child = left_child_ref.take().unwrap();
        let right_child_of_left_child = left_child.right_mut().take();

        // Assume: Tree was had valid AVL heights before
        // Both subtrees of self have valid AVL heights
        *self.left_mut() = right_child_of_left_child;
        self.update_height();

        // Replace self's node reference with the left child
        let old_self = std::mem::replace(&mut self.0, left_child.0);

        // Assume: Tree was had valid AVL heights before
        // Both subtrees of self have valid AVL heights
        *self.right_mut() = Some(SubtreeAnchor(old_self));
        self.update_height();
    }

    /// Performs an AVL single rotation to the left.
    /// Soundness assumption: self.right() is not None.
    fn rotate_left(&mut self) {
        let right_child_ref = self.right_mut();

        if right_child_ref.is_none() {
            panic!("AVL left rotation attempted on a Node with no right child!");
        }

        let mut right_child = right_child_ref.take().unwrap();
        let left_child_of_right_child = right_child.left_mut().take();

        // Assume: Tree was had valid AVL heights before
        // Both subtrees of self have valid AVL heights
        *self.right_mut() = left_child_of_right_child;
        self.update_height();

        // Replace self's node reference with the left child
        let old_self = std::mem::replace(&mut self.0, right_child.0);

        // Assume: Tree was had valid AVL heights before
        // Both subtrees of self have valid AVL heights
        *self.left_mut() = Some(SubtreeAnchor(old_self));
        self.update_height();
    }

    /// Updates height of the subtree based on it's children and performs AVL rotations if needed.
    ///
    /// Soundness assumption: Both children are either empty or have valid heights.
    pub fn balance_subtree(&mut self) {
        self.update_height();

        // If balance is -1, 0 or 1 no need to do anything more
        if self.balance().abs() < 2 {
            return;
        }

        // If it is a left-heavy node
        if self.balance() == -2 {
            // HAS to have a left child, otherwise something's gone terribly wrong
            let left_child = self
                .left_mut()
                .as_mut()
                .expect("Node with AVL balance -2 has empty left child - THIS SHOULD NEVER HAPPEN");

            // If left child is right-heavy:
            // Rotate left child to the left first
            if left_child.balance() > 0 {
                // The left child has to have a right child
                // (because it has to have ANY child, and it is not left-heavy)
                if left_child.right().is_none() {
                    panic!("Right-heavy left-child has no right child - THIS SHOULD NEVER HAPPEN");
                }

                // Case Left-Right
                left_child.rotate_left();
            }

            self.rotate_right();
        } else {
            // It is a right-heavy node then
            // HAS to have a right child, otherwise something's gone terribly wrong
            let right_child = self
                .right_mut()
                .as_mut()
                .expect("Node with AVL balance 2 has empty right child - THIS SHOULD NEVER HAPPEN");

            // If right child is left-heavy:
            // Rotate right child to the right first
            if right_child.balance() < 0 {
                // The right child has to have a left child
                // (because it has to have ANY child, and it is not right-heavy)
                if right_child.left().is_none() {
                    panic!("Left-heavy right-child has no left child - THIS SHOULD NEVER HAPPEN");
                }

                // Case Left-Right
                right_child.rotate_right();
            }

            self.rotate_left();
        }

        self.update_height();
    }

    /// Remove self from the subtree and return a new Subtree rooted at the successor.
    ///
    /// Used during removal. The returned subtree should replace the original root in the bigger tree structure.
    /// After the function finishes, self points to the same node, but is detached from the rest of the tree.
    /// Returned new subtree is valid in terms of AVL.
    /// Successor is the leftmost node of the right subtree.
    /// Soundness assumption: right subtree exists
    fn replace_with_subtree_successor(&mut self) -> SubtreeAnchor<K, V> {
        if self.right().is_none() {
            panic!("Right subtree is empty when taking subtree successor");
        }

        // Unwrap safe due to if above
        let mut right_taken = self.right_mut().take().unwrap();

        // If right child has no left children, it is the immediate successor - no stack needed
        if right_taken.left().is_none() {
            let saved_left = self.left_mut().take();

            // Save left subtree of the removed root in the successor
            *right_taken.left_mut() = saved_left;

            // Restore AVL balance after modifications.
            self.balance_subtree();
            right_taken.balance_subtree();

            return right_taken;
        }

        // Right child has left subtree - descend
        // unwrap safe due to if above
        let mut next_node = right_taken.left_mut().take().unwrap();
        let mut node_stack = Vec::from([right_taken]);

        // Next node points at the next NullableNodeRef, but we're guaranteed that it is Some
        // As long as that Node has a left child, we descend one level further
        while next_node.left().is_some() {
            let next_left = next_node.left_mut().take().unwrap();
            node_stack.push(next_node);
            next_node = next_left;
        }

        // Remove the successor from the subtree and save it's right subtree.
        // Variable is named left_subtree, since it will be attached to successors parent on the left.
        let mut taken_successor = next_node;
        let mut left_subtree = taken_successor.right_mut().take();

        // Ascend on the stack one by one fixing every node
        while let Some(mut parent_node) = node_stack.pop() {
            // Append left subtree on the left of the parent node
            *parent_node.left_mut() = left_subtree;

            // Fix parent node's balance
            parent_node.balance_subtree();

            // Assign parent node to the next subtree
            left_subtree = Some(parent_node);
        }

        // In the end, attach self's original subtrees to the successor.
        // "left_subtree" becomes the right subtree, since it was rebuilt from successors ancestors.
        *taken_successor.left_mut() = self.left_mut().take();
        *taken_successor.right_mut() = left_subtree;

        // Return the successor, as it is a new root for the subtree
        taken_successor
    }

    /// Remove self from the subtree and return a new Subtree rooted at the predecessor.
    ///
    /// Used during removal. The returned subtree should replace the original root in the bigger tree structure.
    /// After the function finishes, self points to the same node, but is detached from the rest of the tree.
    /// Returned new subtree is valid in terms of AVL.
    /// Predecessor is the rightmost node of the left subtree.
    /// Soundness assumption: left subtree exists
    fn replace_with_subtree_predecessor(&mut self) -> SubtreeAnchor<K, V> {
        if self.left().is_none() {
            panic!("Left subtree is empty when taking subtree successor");
        }

        // Unwrap safe due to if above
        let mut left_taken = self.left_mut().take().unwrap();

        // If left child has no right children, it is the immediate successor - no stack needed
        if left_taken.right().is_none() {
            let saved_right = self.right_mut().take();

            // Save right subtree of the removed root in the predecessor
            *left_taken.left_mut() = saved_right;

            // Restore AVL balance after modifications.
            self.balance_subtree();
            left_taken.balance_subtree();

            return left_taken;
        }

        // Left child has right subtree - descend
        // unwrap safe due to if above
        let mut next_node = left_taken.right_mut().take().unwrap();
        let mut node_stack = Vec::from([left_taken]);

        // Next node points at the next NullableNodeRef, but we're guaranteed that it is Some
        // As long as that Node has a right child, we descend one level further
        while next_node.right().is_some() {
            let next_right = next_node.right_mut().take().unwrap();
            node_stack.push(next_node);
            next_node = next_right;
        }

        // Remove the predecessor from the subtree and save it's left subtree.
        // Variable is named right_subtree, since it will be attached to predecessor's parent on the right.
        let mut taken_predecessor = next_node;
        let mut right_subtree = taken_predecessor.left_mut().take();

        // Ascend on the stack one by one fixing every node
        while let Some(mut parent_node) = node_stack.pop() {
            // Append left subtree on the left of the parent node
            *parent_node.right_mut() = right_subtree;

            // Fix parent node's balance
            parent_node.balance_subtree();

            // Assign parent node to the next subtree
            right_subtree = Some(parent_node);
        }

        // In the end, attach self's original subtrees to the successor.
        // "right_subtree" becomes the left subtree, since it was rebuilt from predecessor's ancestors.
        *taken_predecessor.right_mut() = self.right_mut().take();
        *taken_predecessor.left_mut() = right_subtree;

        // Return the predecessor as it is a new root for the subtree
        taken_predecessor
    }

    // replaces just the value of the node
    // useful for insertions
    // returns old value
    pub fn replace(&mut self, value: V) -> V {
        std::mem::replace(self.value_mut(), value)
    }
}

impl<K: Display, V: Display> Display for SubtreeAnchor<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "N({} => {}, H({}), BF({}))",
            self.0.key,
            self.0.value,
            self.0.height,
            self.balance()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::SubtreeAnchor;

    #[test]
    fn balance_factor_and_height_are_updated_correctly() {
        //      10
        //     /  \
        //    5    12
        //   /
        //  2

        let mut root = SubtreeAnchor::new_leaf(5, 12);
        root.right_mut().replace(SubtreeAnchor::new_leaf(10, 23));
        root.left_mut().replace(SubtreeAnchor::new_leaf(5, 1111));
        root.left_mut()
            .as_mut()
            .unwrap()
            .left_mut()
            .replace(SubtreeAnchor::new_leaf(2, 1111));

        root.left_mut()
            .as_mut()
            .unwrap()
            .left_mut()
            .as_mut()
            .unwrap()
            .update_height();
        root.left_mut().as_mut().unwrap().update_height();
        root.right_mut().as_mut().unwrap().update_height();
        root.update_height();

        assert_eq!(root.balance(), -1);
        assert_eq!(root.right().as_ref().unwrap().balance(), 0);
        assert_eq!(root.left().as_ref().unwrap().balance(), -1);
        assert_eq!(
            root.left()
                .as_ref()
                .unwrap()
                .left()
                .as_ref()
                .unwrap()
                .balance(),
            0
        );

        assert_eq!(root.0.height, 3);
    }

    #[test]
    fn removal_of_leaf_node() {
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        let replacement_tree = root.remove();

        // Leaf nodes are left empty
        assert!(replacement_tree.is_none());

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn removal_of_node_with_left_child() {
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        const CHILD_KEY: u32 = 2u32;
        const CHILD_VALUE: u32 = 15u32;
        root.left_mut()
            .replace(SubtreeAnchor::new_leaf(CHILD_KEY, CHILD_VALUE));
        root.update_height();

        let replacement_tree = root.remove();

        // Nodes with 1 child are replaced with it
        assert!(replacement_tree.is_some());
        assert_eq!(*replacement_tree.as_ref().unwrap().key(), CHILD_KEY);
        assert_eq!(*replacement_tree.as_ref().unwrap().value(), CHILD_VALUE);

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn removal_of_node_with_right_child() {
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        const CHILD_KEY: u32 = 12u32;
        const CHILD_VALUE: u32 = 15u32;
        root.right_mut()
            .replace(SubtreeAnchor::new_leaf(CHILD_KEY, CHILD_VALUE));
        root.update_height();

        let replacement_tree = root.remove();

        // Nodes with 1 child are replaced with it
        assert!(replacement_tree.is_some());
        assert_eq!(*replacement_tree.as_ref().unwrap().key(), CHILD_KEY);
        assert_eq!(*replacement_tree.as_ref().unwrap().value(), CHILD_VALUE);

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn removal_of_node_with_two_children_left_left_heavy() {
        //      10
        //     /  \
        //    2   12
        //   /
        //  1
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        const PREDECESSOR_KEY: u32 = 2u32;
        const PREDECESSOR_VALUE: u32 = 999u32;
        let left_mut = root.left_mut();
        left_mut.replace(SubtreeAnchor::new_leaf(PREDECESSOR_KEY, PREDECESSOR_VALUE));

        left_mut
            .as_mut()
            .unwrap()
            .left_mut()
            .replace(SubtreeAnchor::new_leaf(1u32, 89u32));
        left_mut.as_mut().unwrap().update_height();

        root.right_mut()
            .replace(SubtreeAnchor::new_leaf(12u32, 15u32));
        root.update_height();

        let replacement_tree = root.remove();

        // Nodes with 1 child are replaced with it
        assert!(replacement_tree.is_some());
        assert_eq!(*replacement_tree.as_ref().unwrap().key(), PREDECESSOR_KEY);
        assert_eq!(
            *replacement_tree.as_ref().unwrap().value(),
            PREDECESSOR_VALUE
        );

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn removal_of_node_with_two_children_left_right_heavy() {
        //      10
        //     /  \
        //    2   12
        //     \
        //      8
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        let left_mut = root.left_mut();
        left_mut.replace(SubtreeAnchor::new_leaf(2u32, 123u32));

        const PREDECESSOR_KEY: u32 = 8u32;
        const PREDECESSOR_VALUE: u32 = 999u32;
        left_mut
            .as_mut()
            .unwrap()
            .right_mut()
            .replace(SubtreeAnchor::new_leaf(PREDECESSOR_KEY, PREDECESSOR_VALUE));
        left_mut.as_mut().unwrap().update_height();

        root.right_mut()
            .replace(SubtreeAnchor::new_leaf(12u32, 15u32));
        root.update_height();

        let replacement_tree = root.remove();

        // Nodes with 1 child are replaced with it
        assert!(replacement_tree.is_some());
        assert_eq!(*replacement_tree.as_ref().unwrap().key(), PREDECESSOR_KEY);
        assert_eq!(
            *replacement_tree.as_ref().unwrap().value(),
            PREDECESSOR_VALUE
        );

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn removal_of_node_with_two_children_right_right_heavy() {
        //      10
        //     /  \
        //    2   12
        //          \
        //          15
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        const SUCCESSOR_KEY: u32 = 12u32;
        const SUCCESSOR_VALUE: u32 = 999u32;
        let right_mut = root.right_mut();
        right_mut.replace(SubtreeAnchor::new_leaf(SUCCESSOR_KEY, SUCCESSOR_VALUE));

        right_mut
            .as_mut()
            .unwrap()
            .right_mut()
            .replace(SubtreeAnchor::new_leaf(15u32, 89u32));
        right_mut.as_mut().unwrap().update_height();

        root.left_mut()
            .replace(SubtreeAnchor::new_leaf(2u32, 12u32));
        root.update_height();

        let replacement_tree = root.remove();

        // Nodes with 1 child are replaced with it
        assert!(replacement_tree.is_some());
        assert_eq!(*replacement_tree.as_ref().unwrap().key(), SUCCESSOR_KEY);
        assert_eq!(*replacement_tree.as_ref().unwrap().value(), SUCCESSOR_VALUE);

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn removal_of_node_with_two_children_right_left_heavy() {
        //      10
        //     /  \
        //    2   15
        //       /
        //      12
        const KEY: u32 = 10u32;
        const VALUE: u32 = 5u32;
        let mut root = SubtreeAnchor::new_leaf(KEY, VALUE);

        let right_mut = root.right_mut();
        right_mut.replace(SubtreeAnchor::new_leaf(15u32, 89u32));

        const SUCCESSOR_KEY: u32 = 15u32;
        const SUCCESSOR_VALUE: u32 = 999u32;
        right_mut
            .as_mut()
            .unwrap()
            .left_mut()
            .replace(SubtreeAnchor::new_leaf(SUCCESSOR_KEY, SUCCESSOR_VALUE));
        right_mut.as_mut().unwrap().update_height();

        root.left_mut()
            .replace(SubtreeAnchor::new_leaf(2u32, 12u32));
        root.update_height();

        let replacement_tree = root.remove();

        // Nodes with 1 child are replaced with it
        assert!(replacement_tree.is_some());
        assert_eq!(*replacement_tree.as_ref().unwrap().key(), SUCCESSOR_KEY);
        assert_eq!(*replacement_tree.as_ref().unwrap().value(), SUCCESSOR_VALUE);

        assert_eq!(*root.key(), KEY);
        assert_eq!(*root.value(), VALUE);
    }

    #[test]
    fn replacement_of_value_keeps_tree_in_tact() {
        //      10
        //     /  \
        //    2   15

        const ORIGINAL_VALUE: u32 = 999;
        const NEW_VALUE: u32 = 1337;
        let mut root = SubtreeAnchor::new_leaf(10u32, ORIGINAL_VALUE);

        const KEY_LEFT: u32 = 2u32;
        const KEY_RIGHT: u32 = 15u32;
        root.left_mut()
            .replace(SubtreeAnchor::new_leaf(KEY_LEFT, 123u32));
        root.right_mut()
            .replace(SubtreeAnchor::new_leaf(KEY_RIGHT, 122u32));
        root.update_height();

        let result = root.replace(NEW_VALUE);

        assert_eq!(result, ORIGINAL_VALUE);
        assert_eq!(*root.value(), NEW_VALUE);
        assert!(root.left().is_some());
        assert!(root.right().is_some());
        assert_eq!(*root.left().as_ref().unwrap().key(), KEY_LEFT);
        assert_eq!(*root.right().as_ref().unwrap().key(), KEY_RIGHT);
    }
}
