// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_map;
use crate::identity_set::IdentitySet;

use core::fmt::{self, Debug, Formatter};
use allocator_api2::alloc::{Allocator, Global};
use core::iter::FusedIterator;
use core::ptr;

/// Owning identity set iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone)]
pub struct IntoIter<T, A: Allocator = Global> {
	iter: identity_map::IntoIter<T, (), A>,
}

impl<T, A: Allocator> IntoIter<T, A> {
	/// Constructs a new, owning identity set iterator.
	#[inline(always)]
	pub(crate) fn new(set: IdentitySet<T, A>) -> Self {
		let iter = set.into_map().into_iter();
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

impl<T, A> Debug for IntoIter<T, A>
where
	T: Debug,
	A: Allocator,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f
			.debug_tuple("IntoIter")
			.field(&self.as_slice())
			.finish()
	}
}

impl<T, A: Allocator + Default> Default for IntoIter<T, A> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<T, A: Allocator> Iterator for IntoIter<T, A> {
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

impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}
}

impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> { }

impl<T, A: Allocator> FusedIterator for IntoIter<T, A> { }
