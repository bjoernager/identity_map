# Changelog

This is the changelog of [identity_map](https://crates.io/crates/identity_map/).
See `README.md` for more information.

## 0.2.0

* Implement `From<{[(K, V); N]}>` for `IdentityMap<K, V>`
* Add readme
* Disable `allocator_api` (replace with `allocator_api2` crate)
* Decrease Rust edition to `2021`
* Decrease MSRV to `1.82`
* Update tests
* Add `IdentitySet` type
* Implement `Hash`, `IntoIterator`, `Clone`, and `Default` for `IdentitySet`
* Add `identity_map` and `identity_set` modules
* Clone existing `IntoIter`, `Iter`, and `IterMut` iterators into `identity_set`
* Improve commenting
* Implement `From<[K; _]>` for `IdentitySet<K, _>`
* Add `new`, `new_in`, `with_capacity`, `with_capacity_in`, `from_raw_parts`, and `from_raw_parts_in` constructors to `IdentitySet`
* Add `into_raw_parts` and `into_raw_parts_with_alloc` destructors to `IdentitySet`
* Add `iter` and `iter_mut` methods to `IdentitySet`
* Add `len`, `is_empty`, and `capacity` methods to `IdentitySet`
* Add `as_ptr`, `as_mut_ptr`, `as_slice`, and `as_mut_slice` methods to `IdentitySet`
* Add `insert`, `remove`, and `contains` methods to `IdentitySet`
* Implement `IntoIterator` for `&IdentitySet` and `&mut IdentitySet`
* Implement `Send` and `Sync` for `{identity_map, identity_set}::Iter`
* Unimplement `AsRef<[(K, V)]>` for `IdentityMap` and `identity_map::{IntoIter, Iter, IterMut}`
* Unimplement `AsMut<[(K, V)]>` for `IdentityMap` and `identity_map::{IntoIter, IterMut}`
* Remove `Eq` bound from some `IdentityMap` implementation
* Update lints
* Update docs

## 0.1.0

* Implement `Clone` for `IdentityMap` and `IntoIter`
* Unimplement `Clone` for `IterMut`
* Add `as_slice` method to `IntoIter`, `Iter`, and `IterMut`
* Add `as_mut_slice` method to `IntoIter` and `IterMut`
* Fix `IntoIter` not tracking allocation state
* Fix `IdentityMap` not properly tracking allocation state
* Improve commenting
* Implement `Send` and `Sync` for `IntoIter`
* Add `as_ptr` and `as_mut_ptr` methods to `IntoIter`, `Iter`, and `IterMut`
* Add more docs
* Update lints
* Properly implement `<{IntoIter, Iter, IterMut} as Iterator>::size_hint`
* Implement `AsRef<[(K, V)]>` for `IdentityMap`, `IntoIter`, `Iter` and `IterMut`
* Implement `AsMut<[(K, V)]>` for `IdentityMap`, `IntoIter`, and `IterMut`

## 0.0.1

* Add license file
* Add crate description

## 0.0.0

* Add gitignore
* Add changelog
* Add Cargo manifest
* Configure lints
* Enable `allocator_api`
* Add `IdentityMap` type
* Implement `Default`, `IntoIterator`, `Hash`, `Index`, and `IndexMut` for `IdentityMap`
* Implement `IntoIterator` for `&IdentityMap` and `&mut IdentityMap`
* Add `new` and `new_in` constructors to `IdentityMap`
* Add `with_capacity` and `with_capacity_in` constructors to `IdentityMap`
* Add `allocator` method to `IdentityMap`
* Add `iter` and `iter_mut` methods to `IdentityMap`
* Add `get` and `get_mut` methods to `IdentityMap`
* Add `capacity`, `len` and `is_emtpy` methods to `IdentityMap`
* Add `insert` and `remove` methods to `IdentityMap`
* Add `contains` method to `IdentityMap`
* Add `as_ptr` and `as_mut_ptr` methods to `IdentityMap`
* Add `reserve` method to `IdentityMap`
* Add `IntoIter`, `Iter`, and `IterMut` iterators
* Implement `Default`, `Iterator`, `DoubleEndedIterator`, `ExactSizedIterator`, and `FusedIterator` for `IntoIter`, `Iter`, and `IterMut`
* Add tests
* Add `into_raw_parts` and `into_raw_parts_with_alloc` deconstructors to `IdentityMap`
* Add `as_slice` and `as_mut_slice` methods to `IdentityMap`
* Implement `Send` and `Sync` for `IdentityMap`
* Add `from_raw_parts` and `from_raw_parts_in` constructors to `IdentityMap`
* License under an MIT licence
