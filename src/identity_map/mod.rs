// Copyright 2025 Gabriel Bjørnager Jensen.

//! The [`IdentityMap`] type and associated facilities.

mod drain;
mod identity_map;
mod into_iter;
mod into_keys;
mod into_values;
mod iter;
mod iter_mut;
mod keys;
mod values;
mod values_mut;

pub use drain::Drain;
pub use identity_map::IdentityMap;
pub use into_iter::IntoIter;
pub use into_keys::IntoKeys;
pub use into_values::IntoValues;
pub use iter::Iter;
pub use iter_mut::IterMut;
pub use keys::Keys;
pub use values::Values;
pub use values_mut::ValuesMut;
