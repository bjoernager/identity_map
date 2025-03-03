// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::{IdentityMap, Iter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Borrowing identity map values iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Values<'a, K, V> {
	iter: Iter<'a, K, V>,
}

impl<'a, K, V> Values<'a, K, V> {
	/// Constructs a new, borrowing identity map values iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a IdentityMap<K, V, A>) -> Self {
		let iter = map.iter();
		Self { iter }
	}
}

impl<K, V> Default for Values<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for Values<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(_, v)| v)
	}
}

impl<K, V> ExactSizeIterator for Values<'_, K, V> { }

impl<K, V> FusedIterator for Values<'_, K, V> { }

impl<'a, K, V> Iterator for Values<'a, K, V> {
	type Item = &'a V;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(_, v)| v)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
