// Copyright 2025 Gabriel Bjørnager Jensen.

#![doc(html_logo_url = "https://gitlab.com/bjoernager/identity_map/-/raw/master/doc-icon.svg")]

//! `identity_map` is a Rust crate for mapping keys with associated values.
//!
//! This crate defines the [`IdentityMap`] and [`IdentitySet`] as analogues to the standard library's [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet).
//! Contrary to the standard library, however, keys are in the identity collections transformed as if by using [the identity function](https://en.wikipedia.org/wiki/Identity_function/).
//!
//! Using the identity function *may* make tables larger in size (depending on the key type), but does also allow for making the very same tables non-collidable.
//! The collections provided by this crate are ordered and required keys implementing [`Ord`].
//!
//! # Copyright & Licence.
//!
//! Copyright 2025 Gabriel Bjørnager Jensen.
//!
//! `identity_map` is distributed under either an MIT licence or version 2.0 of the Apache License, at your option.

#![no_std]

#![cfg_attr(feature = "allocator_api", feature(allocator_api))]

#![warn(missing_docs)]

extern crate alloc;

#[cfg(doc)]
extern crate std;

pub mod identity_map;
pub mod identity_set;

#[doc(inline)]
pub use crate::identity_map::IdentityMap;

#[doc(inline)]
pub use crate::identity_set::IdentitySet;
