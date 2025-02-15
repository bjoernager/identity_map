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

use crate::identity_set::IdentitySet;

use allocator_api2::alloc::Allocator;
use core::any::type_name;
use core::fmt::{self, Formatter};
use core::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};

#[repr(transparent)]
#[derive(Default)]
struct IdentitySetVisitor<T, A: Allocator> {
	_set: PhantomData<fn() -> IdentitySet<T, A>>,
}

impl<T, A: Allocator> IdentitySetVisitor<T, A> {
	#[inline(always)]
	#[must_use]
	pub const fn new() -> Self {
		Self { _set: PhantomData }
	}
}

impl<'de, T, Alloc> Visitor<'de> for IdentitySetVisitor<T, Alloc>
where
	T:     Deserialize<'de> + Ord,
	Alloc: Allocator + Default,
{
	type Value = IdentitySet<T, Alloc>;

	#[inline(always)]
	fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
		let key_name = type_name::<T>();

		write!(formatter, "an identity set of `{key_name}`")
	}

	#[inline]
	fn visit_seq<A: SeqAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
		let alloc = Default::default();
		let cap   = map.size_hint().unwrap_or_default();

		let mut this = IdentitySet::with_capacity_in(cap, alloc);

		while let Some(key) = map.next_element()? {
			this.insert(key);
		}

		Ok(this)
	}
}

impl<'de, T, A> Deserialize<'de> for IdentitySet<T, A>
where
	T: Deserialize<'de> + Ord,
	A: Allocator + Default,
{
	#[inline(always)]
	fn deserialize<D: Deserializer<'de>>(deserialiser: D) -> Result<Self, D::Error> {
		deserialiser.deserialize_seq(IdentitySetVisitor::<T, A>::new())
	}
}

impl<T, A> Serialize for IdentitySet<T, A>
where
	T: Serialize,
	A: Allocator,
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serialiser: S) -> Result<S::Ok, S::Error> {
		serialiser.collect_seq(self.iter())
	}
}
