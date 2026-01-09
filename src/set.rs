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

use crate::map::BSTMap;

/// Ordered Set based on a self-balancing AVL Binary Search Tree.
pub struct BSTSet<K: Ord> {
    map: BSTMap<K, ()>
}

impl<K: Ord> BSTSet<K> {
    /// Creates a new, empty BSTSet.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::set::BSTSet;
    ///
    /// // You most likely want to keep it mut to modify it
    /// let mut set: BSTSet<u32> = BSTSet::new();
    /// ```
    pub fn new() -> Self {
        Self {
            map: BSTMap::new()
        }
    }

    /// Returns current length of the BSTSet (number of elements).
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if BSTSet is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Drops all data nodes from the BSTSet, making it empty afterwards.
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Inserts a new key into the BSTSet.
    /// If the key already exists no new data nodes are allocated, and the function returns `false`.
    /// Otherwise, `true` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::set::BSTSet;
    ///
    /// let mut set: BSTSet<u32> = BSTSet::new();
    /// set.insert(32);
    ///
    /// let exists1 = set.insert(90);
    /// let exists2 = set.insert(90);
    ///
    /// assert!(exists1);
    /// assert!(!exists2);
    /// ```
    pub fn insert(&mut self, key: K) -> bool {
        self.map.insert(key, ()).is_none()
    }

    /// Returns `true` if a given key exists in the BSTSet, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::set::BSTSet;
    ///
    /// let mut set: BSTSet<u32> = BSTSet::new();
    /// set.insert(32);
    ///
    /// assert!(set.contains(&32));
    /// assert!(!set.contains(&999));
    /// ```
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains(key)
    }
    
    /// Removes given key from the BSTSet and returns `true` if given key existed.
    /// If given key does not exist in the BSTMap, the method does nothing and returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbst328::set::BSTSet;
    ///
    /// let mut set: BSTSet<u32> = BSTSet::new();
    /// set.insert(32);
    ///
    /// let removed1 = set.remove(&90);
    /// let removed2 = set.remove(&32);
    /// let removed3 = set.remove(&32);
    ///
    /// assert!(!removed1);
    /// assert!(removed2);
    /// assert!(!removed3);
    /// ```
    pub fn remove(&mut self, key: &K) -> bool {
        self.map.remove(key).is_some()
    }
}