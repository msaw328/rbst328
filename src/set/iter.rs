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
