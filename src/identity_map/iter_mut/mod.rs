// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::IdentityMap;

use allocator_api2::alloc::Allocator;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::slice;

/// Mutably-borrowing identity map iterator.
#[must_use]
pub struct IterMut<'a, K, V> {
	iter: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> IterMut<'a, K, V> {
	/// Constructs a new, mutably-borrowing identity map iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a mut IdentityMap<K, V, A>) -> Self {
		let iter = map.as_mut_slice().iter_mut();
		Self { iter }
	}

	/// Gets a slice of the key-value pairs.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_slice(&self) -> &[(K, V)] {
		self.iter.as_slice()
	}
}

impl<K, V> Debug for IterMut<'_, K, V>
where
	K: Debug,
	V: Debug,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f
			.debug_tuple("IterMut")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, V> Default for IterMut<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, v)| (&*k, v))
	}
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> { }

impl<K, V> FusedIterator for IterMut<'_, K, V> { }

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
	type Item = (&'a K, &'a mut V);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, v)| (&*k, v))
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
