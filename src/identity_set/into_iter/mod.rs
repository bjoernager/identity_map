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
use core::ptr::{self, drop_in_place};

/// Owning identity set iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Default)]
pub struct IntoIter<K, A: Allocator = Global> {
	iter: identity_map::IntoIter<K, (), A>,
}

impl<K, A: Allocator> IntoIter<K, A> {
	/// Constructs a new, owning identity set iterator.
	///
	/// # Safety
	///
	/// The provided, raw identity set must be initialised.
	#[inline(always)]
	pub(crate) fn new(set: IdentitySet<K, A>) -> Self {
		let map = set.into_identity_map();

		let iter = identity_map::IntoIter::new(map);
		Self { iter }
	}

	/// Gets a pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const K {
		// SAFETY: `(K, ())` is transparent to `K`.
		self.iter.as_ptr() as *const K
	}

	/// Gets a mutable pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut K {
		// SAFETY: `(K, ())` is transparent to `K`.
		self.iter.as_mut_ptr() as *mut K
	}

	/// Gets a slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[K] {
		// SAFETY: `(K, ())` is transparent to `K`.
		unsafe { &*(&raw const *self.iter.as_slice() as *const [K]) }
	}

	/// Gets a mutable slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [K] {
		// SAFETY: `(K, ())` is transparent to `K`.
		unsafe { &mut *(&raw mut *self.iter.as_mut_slice() as *mut [K]) }
	}
}

impl<K, A> Debug for IntoIter<K, A>
where
	K: Debug,
	A: Allocator,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("IterMut")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, A: Allocator> Drop for IntoIter<K, A> {
	#[inline(always)]
	fn drop(&mut self) {
		// Drop all items that are still alive.

		let remaining = ptr::from_mut(self.as_mut_slice());
		unsafe { drop_in_place(remaining) };
	}
}

impl<K, A: Allocator> Iterator for IntoIter<K, A> {
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

impl<K, A: Allocator> DoubleEndedIterator for IntoIter<K, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<K, A: Allocator> ExactSizeIterator for IntoIter<K, A> { }

impl<K, A: Allocator> FusedIterator for IntoIter<K, A> { }
