// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::{IdentityMap, Iter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Borrowing identity map keys iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Keys<'a, K, V> {
	iter: Iter<'a, K, V>,
}

impl<'a, K, V> Keys<'a, K, V> {
	/// Constructs a new, borrowing identity map keys iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a IdentityMap<K, V, A>) -> Self {
		let iter = map.iter();
		Self { iter }
	}
}

impl<K, V> Default for Keys<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for Keys<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<K, V> ExactSizeIterator for Keys<'_, K, V> { }

impl<K, V> FusedIterator for Keys<'_, K, V> { }

impl<'a, K, V> Iterator for Keys<'a, K, V> {
	type Item = &'a K;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
