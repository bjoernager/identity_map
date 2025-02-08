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

/// Mutably-borrowing identity set iterator.
#[must_use]
#[repr(transparent)]
#[derive(Default)]
pub struct IterMut<'a, K> {
	iter: identity_map::IterMut<'a, K, ()>,
}

impl<'a, K> IterMut<'a, K> {
	/// Constructs a new, mutably-borrowing identity set iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(set: &mut IdentitySet<K, A>) -> Self {
		let iter = identity_map::IterMut::new(set.as_mut_identity_map());
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
	pub fn as_slice(&self) -> &'a [K] {
		// SAFETY: `(K, ())` is transparent to `K`.
		unsafe { &*(&raw const *self.iter.as_slice() as *const [K]) }
	}

	/// Gets a mutable slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &'a mut [K] {
		// SAFETY: `(K, ())` is transparent to `K`.
		unsafe { &mut *(&raw mut *self.iter.as_mut_slice() as *mut [K]) }
	}
}

impl<K: Debug> Debug for IterMut<'_, K> {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("IterMut")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K> DoubleEndedIterator for IterMut<'_, K> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<K> ExactSizeIterator for IterMut<'_, K> { }

impl<K> FusedIterator for IterMut<'_, K> { }

impl<'a, K> Iterator for IterMut<'a, K> {
	type Item = &'a mut K;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

// SAFETY: The internal pointer is guaranteed to
// be exclusive.
unsafe impl<K: Send> Send for IterMut<'_, K> { }

// SAFETY: The internal pointer is guaranteed to
// be exclusive.
unsafe impl<K: Sync> Sync for IterMut<'_, K> { }
