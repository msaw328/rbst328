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

//! Implementation for `serde` traits for the `BSTMap` type. Available only with the `serde` feature flag

// Code here has been copied from:
// https://serde.rs/deserialize-map.html
// https://serde.rs/impl-serialize.html#serializing-a-sequence-or-map

use std::{fmt, marker::PhantomData};

use serde_core::{
    Deserialize, Deserializer,
    de::{MapAccess, Visitor},
    ser::{Serialize, SerializeMap},
};

use crate::map::BSTMap;

impl<K: Serialize + Ord, V: Serialize> Serialize for BSTMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde_core::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

// A Visitor is a type that holds methods that a Deserializer can drive
// depending on what is contained in the input data.
//
// In the case of a map we need generic type parameters K and V to be
// able to set the output type correctly, but don't require any state.
// This is an example of a "zero sized type" in Rust. The PhantomData
// keeps the compiler from complaining about unused generic type
// parameters.
struct BSTMapVisitor<K: Ord, V> {
    marker: PhantomData<fn() -> BSTMap<K, V>>,
}

impl<K: Ord, V> BSTMapVisitor<K, V> {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

// This is the trait that Deserializers are going to be driving. There
// is one method for each type of data that our type knows how to
// deserialize from. There are many other methods that are not
// implemented here, for example deserializing from integers or strings.
// By default those methods will return an error, which makes sense
// because we cannot deserialize a MyMap from an integer or string.
impl<'de, K: Ord, V> Visitor<'de> for BSTMapVisitor<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    // The type that our Visitor is going to produce.
    type Value = BSTMap<K, V>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("BSTMap")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BSTMap::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}

// This is the trait that informs Serde how to deserialize MyMap.
impl<'de, K: Ord, V> Deserialize<'de> for BSTMap<K, V>
where
    K: Deserialize<'de>,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of MyMap.
        deserializer.deserialize_map(BSTMapVisitor::new())
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{Token, assert_tokens};

    use crate::map::BSTMap;

    #[test]
    fn empty_map_to_tokens() {
        let map: BSTMap<i32, u32> = BSTMap::new();

        assert_tokens(&map, &[Token::Map { len: Some(0) }, Token::MapEnd]);
    }

    #[test]
    fn nonempty_map_to_tokens() {
        let mut map: BSTMap<i32, &str> = BSTMap::new();
        map.insert(32, "test");
        map.insert(89, "test1");
        map.insert(23, "test2");

        // Sorted order!
        assert_tokens(
            &map,
            &[
                Token::Map { len: Some(3) },
                Token::I32(23),
                Token::BorrowedStr("test2"),
                Token::I32(32),
                Token::BorrowedStr("test"),
                Token::I32(89),
                Token::BorrowedStr("test1"),
                Token::MapEnd,
            ],
        );
    }
}
