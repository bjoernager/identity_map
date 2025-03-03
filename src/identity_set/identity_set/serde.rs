// Copyright 2025 Gabriel Bjørnager Jensen.

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
