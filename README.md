# `identity_map`

`identity_map` is a Rust crate for mapping keys with associated values.

This crate defines the `IdentityMap` and `IdentitySet` as analogues to the standard library's `HashMap` and `HashSet`.
Contrary to the standard library, however, keys are in the identity collections transformed as if by using [the identity function](https://en.wikipedia.org/wiki/Identity_function/).

Using the identity function *may* make tables larger in size (depending on the key type), but does also allow for making the very same tables non-collidable.
The collections provided by this crate are ordered and required keys implementing `Ord`.

## Copyright & Licence.

Copyright 2025 Gabriel Bjørnager Jensen.

`identity_map` is distributed under either an MIT licence (see `LICENCE-MIT`) or version 2.0 of the Apache License (see `LICENCE-APACHE`), at your option.
