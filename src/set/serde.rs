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

//! Implementation for `serde` traits for the `BSTSet` type. Available only with the `serde` feature flag

// Code here has been copied from:
// https://serde.rs/deserialize-map.html
// https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map
// And modified to support seq instead

use std::{fmt, marker::PhantomData};

use serde_core::{
    Deserialize, Deserializer,
    de::{SeqAccess, Visitor},
    ser::{Serialize, SerializeSeq},
};

use crate::set::BSTSet;

impl<K: Serialize + Ord> Serialize for BSTSet<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for k in self.iter() {
            seq.serialize_element(k)?;
        }
        seq.end()
    }
}

// A Visitor is a type that holds methods that a Deserializer can drive
// depending on what is contained in the input data.
struct BSTSetVisitor<K: Ord> {
    marker: PhantomData<fn() -> BSTSet<K>>,
}

impl<K: Ord> BSTSetVisitor<K> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de, K: Ord> Visitor<'de> for BSTSetVisitor<K>
where
    K: Deserialize<'de>,
{
    // The type that our Visitor is going to produce.
    type Value = BSTSet<K>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("BSTSet")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut set = BSTSet::new();

        while let Some(k) = access.next_element()? {
            set.insert(k);
        }

        Ok(set)
    }
}

impl<'de, K: Ord> Deserialize<'de> for BSTSet<K>
where
    K: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BSTSetVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_tokens};

    use crate::set::BSTSet;

    #[test]
    fn empty_set_to_tokens() {
        let set: BSTSet<i32> = BSTSet::new();

        assert_tokens(&set, &[Token::Seq { len: Some(0) }, Token::SeqEnd]);
    }

    #[test]
    fn nonempty_set_to_tokens() {
        let mut set: BSTSet<i32> = BSTSet::new();
        set.insert(32);
        set.insert(89);
        set.insert(23);

        // Sorted order!
        assert_tokens(&set, &[
            Token::Seq { len: Some(3) },

            Token::I32(23),
   
            Token::I32(32),

            Token::I32(89),

            Token::SeqEnd
        ]);
    }
}