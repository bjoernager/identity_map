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
	///
	/// # Safety
	///
	/// The provided, raw identity set must be initialised.
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
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
