// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::IdentityMap;

use allocator_api2::alloc::Allocator;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::slice;

/// Borrowing identity map iterator.
#[must_use]
#[derive(Clone)]
pub struct Iter<'a, K, V> {
	iter: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iter<'a, K, V> {
	/// Constructs a new, borrowing identity map iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a IdentityMap<K, V, A>) -> Self {
		let iter = map.as_slice().iter();
		Self { iter }
	}

	/// Gets a slice of the key-value pairs.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_slice(&self) -> &[(K, V)] {
		self.iter.as_slice()
	}
}

impl<K, V> Debug for Iter<'_, K, V>
where
	K: Debug,
	V: Debug,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f
			.debug_tuple("Iter")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, V> Default for Iter<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, v)| (k, v))
	}
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> { }

impl<K, V> FusedIterator for Iter<'_, K, V> { }

impl<'a, K, V> Iterator for Iter<'a, K, V> {
	type Item = (&'a K, &'a V);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, v)| (k, v))
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
