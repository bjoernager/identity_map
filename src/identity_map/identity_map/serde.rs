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

use crate::identity_map::IdentityMap;

use allocator_api2::alloc::Allocator;
use core::any::type_name;
use core::fmt::{self, Formatter};
use core::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};

#[expect(clippy::type_complexity)]
#[repr(transparent)]
#[derive(Default)]
struct IdentityMapVisitor<K, V, A: Allocator> {
	_map: PhantomData<fn() -> IdentityMap<K, V, A>>,
}

impl<K, V, A: Allocator> IdentityMapVisitor<K, V, A> {
	#[inline(always)]
	#[must_use]
	pub const fn new() -> Self {
		Self { _map: PhantomData }
	}
}

impl<'de, K, V, Alloc> Visitor<'de> for IdentityMapVisitor<K, V, Alloc>
where
	K:     Deserialize<'de> + Ord,
	V:     Deserialize<'de>,
	Alloc: Allocator + Default,
{
	type Value = IdentityMap<K, V, Alloc>;

	#[inline(always)]
	fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
		let key_name   = type_name::<K>();
		let value_name = type_name::<V>();

		write!(formatter, "an identity map between `{key_name}` and `{value_name}`")
	}

	#[inline]
	fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
		let alloc = Default::default();
		let cap   = map.size_hint().unwrap_or_default();

		let mut this = IdentityMap::with_capacity_in(cap, alloc);

		while let Some((key, value)) = map.next_entry()? {
			this.insert(key, value);
		}

		Ok(this)
	}
}

impl<'de, K, V, A> Deserialize<'de> for IdentityMap<K, V, A>
where
	K: Deserialize<'de> + Ord,
	V: Deserialize<'de>,
	A: Allocator + Default,
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserialiser: D) -> Result<Self, D::Error> {
		deserialiser.deserialize_map(IdentityMapVisitor::<K, V, A>::new())
	}
}

impl<K, V, A> Serialize for IdentityMap<K, V, A>
where
	K: Serialize,
	V: Serialize,
	A: Allocator,
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serialiser: S) -> Result<S::Ok, S::Error> {
		serialiser.collect_map(self.iter())
	}
}
