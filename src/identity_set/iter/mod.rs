// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map;
use crate::identity_set::IdentitySet;

use allocator_api2::alloc::Allocator;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::ptr;

/// Borrowing identity set iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone)]
pub struct Iter<'a, T> {
	iter: identity_map::Iter<'a, T, ()>,
}

impl<'a, T> Iter<'a, T> {
	/// Constructs a new, borrowing identity set iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(set: &'a IdentitySet<T, A>) -> Self {
		let iter = set.as_map().iter();
		Self { iter }
	}

	/// Gets a slice of the key-value pairs.
	#[inline(always)]
	pub(crate) fn as_slice(&self) -> &[T] {
		let ptr = ptr::from_ref(self.iter.as_slice()) as *const [T];

		// SAFETY: `(T, ())` is transparent to `T`.
		unsafe { &*ptr }
	}
}

impl<T: Debug> Debug for Iter<'_, T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("Iter").field(&self.as_slice()).finish()
	}
}

impl<T> Default for Iter<'_, T> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<T> ExactSizeIterator for Iter<'_, T> { }

impl<T> FusedIterator for Iter<'_, T> { }

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
