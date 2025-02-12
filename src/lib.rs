// Copyright 2025 Gabriel Bjørnager Jensen.
//
// Permission is hereby granted, free of charge, to
// any person obtaining a copy of this software and
// associated documentation files (the "Software"),
// to deal in the Software without restriction, in-
// cluding without limitation the rights to use,
// copy, modify, merge, publish, distribute, subli-
// cense, and/or sell copies of the Software, and
// to permit persons to whom the Software is fur-
// nished to do so, subject to the following condi-
// tions:
//
// The above copyright notice and this permission
// notice shall be included in all copies or sub-
// stantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WAR-
// RANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUD-
// ING BUT NOT LIMITED TO THE WARRANTIES OF MER-
// CHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE,
// AND NONINFRINGEMENT. IN NO EVENT SHALL THE AU-
// THORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! `identity_map` is a Rust crate for mapping keys with associated values.
//!
//! This crate defines the [`IdentityMap`] and [`IdentitySet`] as analogues to the standard library's [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet).
//! Contrary to the standard library, however, keys are in the identity collections transformed as if by using [the identity function](https://en.wikipedia.org/wiki/Identity_function/).
//!
//! Using the identity function *may* make tables larger in size (depending on the key type), but does also allow for making the very same tables non-collidable.
//! The collections provided by this crate are ordered and required keys implementing [`Ord`] (as well as [`Eq`]).
//!
//! # Copyright & License.
//!
//! Copyright 2025 Gabriel Bjørnager Jensen.
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![no_std]

#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

#![warn(missing_docs)]
#![forbid(rust_2024_compatibility)]

extern crate alloc;

#[cfg(doc)]
extern crate std;

/// Includes a module and imports it contents.
///
/// The provided visibility denotes the visibility of **all** imported items.
macro_rules! use_mod {
	($vis:vis $name:ident$(,)?) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

pub mod identity_map;
pub mod identity_set;

#[doc(inline)]
pub use crate::identity_map::IdentityMap;

#[doc(inline)]
pub use crate::identity_set::IdentitySet;
