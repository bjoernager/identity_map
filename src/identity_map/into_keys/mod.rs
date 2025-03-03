// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::{IdentityMap, IntoIter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Owning identity map keys iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct IntoKeys<K, V, A: Allocator> {
	iter: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator> IntoKeys<K, V, A> {
	/// Constructs a new, owning identity map keys iterator.
	#[inline(always)]
	pub(crate) fn new(map: IdentityMap<K, V, A>) -> Self {
		let iter = map.into_iter();
		Self { iter }
	}
}

impl<K, V, A: Allocator + Default> Default for IntoKeys<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V, A: Allocator> DoubleEndedIterator for IntoKeys<K, V, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoKeys<K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for IntoKeys<K, V, A> { }

impl<K, V, A: Allocator> Iterator for IntoKeys<K, V, A> {
	type Item = K;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
