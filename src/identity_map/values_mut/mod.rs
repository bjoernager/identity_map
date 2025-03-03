// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map::{IdentityMap, IterMut};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Mutably-borrowing identity map values iterator.
#[must_use]
#[repr(transparent)]
#[derive(Debug)]
pub struct ValuesMut<'a, K, V> {
	iter: IterMut<'a, K, V>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
	/// Constructs a new, mutably-borrowing identity map values iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a mut IdentityMap<K, V, A>) -> Self {
		let iter = map.iter_mut();
		Self { iter }
	}
}

impl<K, V> Default for ValuesMut<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(_, v)| v)
	}
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> { }

impl<K, V> FusedIterator for ValuesMut<'_, K, V> { }

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
	type Item = &'a mut V;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(_, v)| v)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
