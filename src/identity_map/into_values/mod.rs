// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::{IdentityMap, IntoIter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Owning identity map values iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct IntoValues<K, V, A: Allocator> {
	iter: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator> IntoValues<K, V, A> {
	/// Constructs a new, owning identity map values iterator.
	#[inline(always)]
	pub(crate) fn new(map: IdentityMap<K, V, A>) -> Self {
		let iter = map.into_iter();
		Self { iter }
	}
}

impl<K, V, A: Allocator + Default> Default for IntoValues<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V, A: Allocator> DoubleEndedIterator for IntoValues<K, V, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(_, v)| v)
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoValues<K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for IntoValues<K, V, A> { }

impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A> {
	type Item = V;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(_, v)| v)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
