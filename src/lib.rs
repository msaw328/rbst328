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

/*!
This crate implements `Map` and `Set` collection types based on an AVL Binary Search Tree.
At the moment the core library uses 100% safe code,
but future unsafe code might be introduced for performance reasons.

# Dependencies and feature flags
Without feature flags enabled, library has no dependencies.
Using the feature flag `serde` enables implementations for
`Serialize` and `Deserialize` on `BSTMap` and `BSTSet` but introduces
dependency on `serde-core`.
*/

mod debug;
pub mod map;
mod node;
pub mod set;
