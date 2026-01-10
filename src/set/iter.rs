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

//! Iterators for the Set data structure.

use super::BSTSet;
use crate::map::iter::{KeysIntoIter, KeysIter};

/// An iterator which iterates by all the keys in order.
///
/// It yields a shared reference to each key.
pub struct InorderIter<'a, K: Ord> {
    inner: KeysIter<'a, K, ()>,
}

impl<'a, K: Ord> InorderIter<'a, K> {
    pub(crate) fn new(bst: &'a BSTSet<K>) -> Self {
        Self {
            inner: bst.map.keys(),
        }
    }
}

impl<'a, K: Ord> Iterator for InorderIter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator which consumes the set and iterates by all the keys in order.
///
/// It yields each key (owned).
pub struct InorderIntoIter<K: Ord> {
    inner: KeysIntoIter<K, ()>,
}

impl<K: Ord> InorderIntoIter<K> {
    pub(crate) fn new(bst: BSTSet<K>) -> Self {
        Self {
            inner: bst.map.into_keys(),
        }
    }
}

impl<K: Ord> Iterator for InorderIntoIter<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, K: Ord> IntoIterator for &'a BSTSet<K> {
    type Item = &'a K;

    type IntoIter = InorderIter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K: Ord> IntoIterator for BSTSet<K> {
    type Item = K;

    type IntoIter = InorderIntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        InorderIntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{BSTSet, InorderIntoIter, InorderIter};

    #[test]
    fn byref_inorder_iter_is_empty_from_empty_map() {
        let bst = BSTSet::<u32>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = InorderIter::new(&bst);
        let next_item = iter.next();

        assert!(next_item.is_none());
    }

    #[test]
    fn byref_inorder_iter_contains_all_items() {
        let mut bst = BSTSet::<u32>::new();

        const SERIES_OF_INSERTIONS: [u32; 5] = [13, 15, 7, 2, 8];

        for k in &SERIES_OF_INSERTIONS {
            bst.insert(*k);
        }

        bst.remove(&7); // remove non-leaf node

        let collected: Vec<&u32> = InorderIter::new(&bst).collect();

        const SERIES_OF_CHECKS: [u32; 4] = [13, 15, 2, 8];

        assert_eq!(collected.len(), bst.len());

        for k in &SERIES_OF_CHECKS {
            assert!(collected.iter().any(|k_iter| *k == **k_iter));
        }
    }

    #[test]
    fn byref_inorder_iter_is_sorted_by_key() {
        let mut bst = BSTSet::<u32>::new();

        const SERIES_OF_INSERTIONS: [u32; 5] = [13, 15, 7, 2, 8];

        for k in &SERIES_OF_INSERTIONS {
            bst.insert(*k);
        }

        let collected: Vec<_> = InorderIter::new(&bst).collect();

        assert!(collected.is_sorted());
    }

    #[test]
    fn consuming_inorder_iter_is_empty_from_empty_map() {
        let bst = BSTSet::<u32>::new();

        assert_eq!(bst.len(), 0);
        assert!(bst.is_empty());

        let mut iter = InorderIntoIter::new(bst);
        let next_item = iter.next();

        assert!(next_item.is_none());
    }

    #[test]
    fn consuming_inorder_iter_contains_all_items() {
        let mut bst = BSTSet::<u32>::new();

        const SERIES_OF_INSERTIONS: [u32; 5] = [13, 15, 7, 2, 8];

        for k in &SERIES_OF_INSERTIONS {
            bst.insert(*k);
        }

        bst.remove(&7); // remove non-leaf node

        let bst_len = bst.len();
        let collected: Vec<u32> = InorderIntoIter::new(bst).collect();

        const SERIES_OF_CHECKS: [u32; 4] = [13, 15, 2, 8];

        assert_eq!(collected.len(), bst_len);

        for k in &SERIES_OF_CHECKS {
            assert!(collected.contains(k));
        }
    }

    #[test]
    fn consuming_inorder_iter_is_sorted_by_key() {
        let mut bst = BSTSet::<u32>::new();

        const SERIES_OF_INSERTIONS: [u32; 5] = [13, 15, 7, 2, 8];

        for k in &SERIES_OF_INSERTIONS {
            bst.insert(*k);
        }

        let collected: Vec<_> = InorderIntoIter::new(bst).collect();

        assert!(collected.is_sorted());
    }
}
