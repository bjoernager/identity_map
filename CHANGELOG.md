# Changelog

This is the changelog of [identity_map](https://crates.io/crates/identity_map/).
See `README.md` for more information.

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
