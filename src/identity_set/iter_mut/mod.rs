// Copyright 2025 Gabriel Bjørnager Jensen.
//
// Permission is hereby granted, free of charge, to
// any person obtaining a copy of this software and
// associated documentation files (the "Software"),
// to deal in the Software without restriction, in-
// cluding without limitation the rights to use,
// copy, modify, merge, publish, distribute, subli-
// cense, and/or sell copies of the Software, and
// to permit persons to whom the Software is fur-
// nished to do so, subject to the following condi-
// tions:
//
// The above copyright notice and this permission
// notice shall be included in all copies or sub-
// stantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WAR-
// RANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUD-
// ING BUT NOT LIMITED TO THE WARRANTIES OF MER-
// CHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE,
// AND NONINFRINGEMENT. IN NO EVENT SHALL THE AU-
// THORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::identity_map;
use crate::identity_set::IdentitySet;

use allocator_api2::alloc::Allocator;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::ptr;

/// Mutably-borrowing identity set iterator.
#[must_use]
#[repr(transparent)]
pub struct IterMut<'a, T> {
	iter: identity_map::IterMut<'a, T, ()>,
}

impl<'a, T> IterMut<'a, T> {
	/// Constructs a new, mutably-borrowing identity set iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(set: &'a mut IdentitySet<T, A>) -> Self {
		let iter = set.as_mut_map().iter_mut();
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

impl<T: Debug> Debug for IterMut<'_, T> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("IterMut")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K> Default for IterMut<'_, K> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<T> ExactSizeIterator for IterMut<'_, T> { }

impl<T> FusedIterator for IterMut<'_, T> { }

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
