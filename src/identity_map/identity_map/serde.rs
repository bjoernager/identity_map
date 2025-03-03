// Copyright 2025 Gabriel Bjørnager Jensen.

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
