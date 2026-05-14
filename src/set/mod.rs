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

//! Functionality related to the Set data structure based on an AVL Binary Search Tree.
//!
//! Main module exports the base data structure, while the `iter` module contains iterators.

pub mod iter;
use iter::*;

#[cfg(feature = "serde")]
pub mod serde;

use crate::map::BSTMap;

use std::fmt;

/// Ordered Set based on a self-balancing AVL Binary Search Tree.
pub struct BSTSet<K: Ord> {
    map: BSTMap<K, ()>,
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
        Self { map: BSTMap::new() }
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

    /// Returns the default in order iterator.
    pub fn iter(&self) -> InorderIter<'_, K> {
        InorderIter::new(self)
    }
}

impl<K: Ord> Default for BSTSet<K> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<K: fmt::Debug + Ord> fmt::Debug for BSTSet<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<K: Ord, const N: usize> From<[K; N]> for BSTSet<K> {
    fn from(array: [K; N]) -> Self {
        let mut set = Self::new();

        for k in array {
            set.insert(k);
        }

        set
    }
}

impl<K: Ord> Extend<K> for BSTSet<K> {
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for k in iter {
            self.insert(k);
        }
    }
}

impl<K: Ord + PartialEq> PartialEq for BSTSet<K> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .all(|k| other.contains(k))
    }
}

#[cfg(test)]
mod tests {
    use super::BSTSet;

    #[test]
    fn new_set_should_be_empty() {
        let set: BSTSet<u32> = BSTSet::new();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn insertion_should_return_false_for_existing_keys() {
        let mut set: BSTSet<u32> = BSTSet::new();

        set.insert(45);

        assert!(!set.insert(45));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insertion_should_return_true_for_new_keys() {
        let mut set: BSTSet<u32> = BSTSet::new();

        assert!(set.insert(45));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn removal_should_return_true_for_existing_keys() {
        let mut set: BSTSet<u32> = BSTSet::new();

        set.insert(45);

        assert!(set.remove(&45));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn removal_should_return_false_for_nonexistent_keys() {
        let mut set: BSTSet<u32> = BSTSet::new();

        assert!(!set.remove(&45));
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn contains_should_return_false_for_nonexistent_keys() {
        let set: BSTSet<u32> = BSTSet::new();

        assert!(!set.contains(&45));
    }

    #[test]
    fn contains_should_return_true_for_existing_keys() {
        let mut set: BSTSet<u32> = BSTSet::new();

        set.insert(45);

        assert!(set.contains(&45));
    }

    #[test]
    fn clear_sets_length_to_zero() {
        let mut set = BSTSet::<u32>::new();

        const KEY: u32 = 1;
        set.insert(KEY);
        set.clear();

        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }
}
