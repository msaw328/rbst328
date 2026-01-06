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

// A reference to a Node/subtree which may be empty
pub type NullableNodeRef<K, V> = Option<NodeRef<K, V>>;

// A reference to a Node/subtree which is guaranteed to be present
#[derive(Debug)]
pub struct NodeRef<K, V>(Box<NodeData<K, V>>);

impl<K, V> NodeRef<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self(Box::new(NodeData {
            left: None,
            right: None,
            height: 1,
            key,
            value,
        }))
    }

    pub fn left(&self) -> &NullableNodeRef<K, V> {
        &self.0.left
    }

    pub fn right(&self) -> &NullableNodeRef<K, V> {
        &self.0.right
    }

    pub fn left_mut(&mut self) -> &mut NullableNodeRef<K, V> {
        &mut self.0.left
    }

    pub fn right_mut(&mut self) -> &mut NullableNodeRef<K, V> {
        &mut self.0.right
    }

    pub fn key(&self) -> &K {
        &self.0.key
    }

    pub fn value(&self) -> &V {
        &self.0.value
    }

    pub fn value_mut(&mut self) -> &mut V {
        &mut self.0.value
    }

    pub fn kv(&self) -> (&K, &V) {
        (&self.0.key, &self.0.value)
    }

    pub fn kv_mut(&mut self) -> (&K, &mut V) {
        (&self.0.key, &mut self.0.value)
    }

    pub fn as_ref(&mut self) -> &NodeData<K, V> {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut NodeData<K, V> {
        &mut self.0
    }

    // Destroys the noderef along with the nodedata
    // Returns the value that was contained within
    // useful for removal
    pub fn consume(self) -> NodeData<K, V> {
        *self.0
    }

    // AVL height of the left subtree
    fn left_height(&self) -> i32 {
        match &self.0.left {
            Some(node) => node.0.height,
            None => 0,
        }
    }

    // AVL height of the right subtree
    fn right_height(&self) -> i32 {
        match &self.0.right {
            Some(node) => node.0.height,
            None => 0,
        }
    }

    // Returns AVL balance factor for given node
    pub fn balance(&self) -> i32 {
        self.right_height() - self.left_height()
    }

    // Updates height of this node based on it's children
    // Soundness assumption: Both children have correct heights/are empty
    fn update_height(&mut self) {
        self.0.height = 1 + self.left_height().max(self.right_height())
    }

    // Performs an AVL single rotation to the right
    // Soundness assumption: self.left() is not None
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
        *self.right_mut() = Some(NodeRef(old_self));
        self.update_height();
    }

    // Performs an AVL single rotation to the left
    // Soundness assumption: self.right() is not None
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
        *self.left_mut() = Some(NodeRef(old_self));
        self.update_height();
    }

    // Updates height of the tree based on it's children
    // and if needed performs AVL rotations to balance the subtree
    // rooted in this node
    //
    // Soundness assumption: Both children are either empty or
    // had balance_subtree() called on them before this call
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

            // If left child is right-heavy or balanced:
            // Rotate left child to the left first
            if left_child.balance() >= 0 {
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

            // If right child is left-heavy or balanced:
            // Rotate right child to the right first
            if right_child.balance() <= 0 {
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

    // Replaces self with successor from children of this node
    // used during removal
    // this is the leftmost node of the right subtree
    // Soundness assumption: right subtree exists
    pub fn replace_with_subtree_successor(&mut self) -> NodeRef<K, V> {
        if self.right().is_none() {
            panic!("Right subtree is empty when taking subtree successor");
        }

        let mut right_taken = self.right_mut().take().unwrap();

        // If right child has no left children, it is the immediate successor - no stack needed
        if right_taken.left().is_none() {
            let saved_left = self.left_mut().take();

            *right_taken.left_mut() = saved_left;

            let old_noderef = std::mem::replace(self, right_taken);

            self.update_height();

            return old_noderef;
        }

        // Right child has left subtree - descend
        let mut next_node = right_taken.left_mut().take().unwrap();
        let mut node_stack = Vec::from([right_taken]);

        // Next node points at the next NullableNodeRef, but we're guaranteed that it is Some
        // As long as that Node has a left child, we descend one level further
        while next_node.left().is_some() {
            let next_left = next_node.left_mut().take().unwrap();
            node_stack.push(next_node);
            next_node = next_left;
        }

        // Move the successor node, and save it's right subtree
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

        // In the end, replace self with successor
        *taken_successor.left_mut() = self.left_mut().take();
        *taken_successor.right_mut() = left_subtree;

        let old_node = std::mem::replace(self, taken_successor);
        self.balance_subtree();
        old_node
    }

    // Replaces self with predecessor from children of this node
    // used during removal
    // this is the rightmost node of the left subtree
    // Soundness assumption: left subtree exists
    pub fn replace_with_subtree_predecessor(&mut self) -> NodeRef<K, V> {
        if self.left().is_none() {
            panic!("Left subtree is empty when taking subtree predecessor");
        }

        let mut left_taken = self.left_mut().take().unwrap();

        // If left child has no right children, it is the immediate successor - no stack needed
        if left_taken.right().is_none() {
            let saved_right = self.right_mut().take();

            *left_taken.left_mut() = saved_right;

            let old_noderef = std::mem::replace(self, left_taken);

            self.update_height();

            return old_noderef;
        }

        // Left child has right subtree - descend
        let mut next_node = left_taken.right_mut().take().unwrap();
        let mut node_stack = Vec::from([left_taken]);

        // Next node points at the next NullableNodeRef, but we're guaranteed that it is Some
        // As long as that Node has a right child, we descend one level further
        while next_node.right().is_some() {
            let next_right = next_node.right_mut().take().unwrap();
            node_stack.push(next_node);
            next_node = next_right;
        }

        // Move the successor node, and save it's right subtree
        let mut taken_successor = next_node;
        let mut right_subtree = taken_successor.left_mut().take();

        // Ascend on the stack one by one fixing every node
        while let Some(mut parent_node) = node_stack.pop() {
            // Append left subtree on the left of the parent node
            *parent_node.right_mut() = right_subtree;

            // Fix parent node's balance
            parent_node.balance_subtree();

            // Assign parent node to the next subtree
            right_subtree = Some(parent_node);
        }

        // In the end, replace self with successor
        *taken_successor.right_mut() = self.right_mut().take();
        *taken_successor.left_mut() = right_subtree;

        let old_node = std::mem::replace(self, taken_successor);
        self.balance_subtree();
        old_node
    }
}

impl<K: Display, V: Display> Display for NodeRef<K, V> {
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

// Data associated with the node
#[derive(Debug)]
pub struct NodeData<K, V> {
    pub left: NullableNodeRef<K, V>,
    pub right: NullableNodeRef<K, V>,
    pub height: i32,
    pub key: K,
    pub value: V,
}
