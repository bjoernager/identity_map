// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map;
use crate::identity_set::IdentitySet;

use core::fmt::{self, Debug, Formatter};
use allocator_api2::alloc::{Allocator, Global};
use core::iter::FusedIterator;
use core::ptr;

/// Identity set drain.
#[must_use]
#[repr(transparent)]
pub struct Drain<'a, T, A: Allocator = Global> {
	iter: identity_map::Drain<'a, T, (), A>,
}

impl<'a, T, A: Allocator> Drain<'a, T, A> {
	/// Constructs a new identity set drain.
	#[inline(always)]
	pub(crate) fn new(set: &'a mut IdentitySet<T, A>) -> Self {
		let iter = set.as_mut_map().drain();
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

impl<T, A> Debug for Drain<'_, T, A>
where
	T: Debug,
	A: Allocator,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f
			.debug_tuple("Drain")
			.field(&self.as_slice())
			.finish()
	}
}

impl<T, A: Allocator> Iterator for Drain<'_, T, A> {
	type Item = T;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}
}

impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A> { }

impl<T, A: Allocator> FusedIterator for Drain<'_, T, A> { }
