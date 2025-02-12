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

use crate::identity_map::IdentityMap;

use allocator_api2::alloc::Allocator;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::slice;

/// Borrowing identity map iterator.
#[must_use]
#[derive(Clone)]
pub struct Iter<'a, K, V> {
	iter: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iter<'a, K, V> {
	/// Constructs a new, borrowing identity map iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a IdentityMap<K, V, A>) -> Self {
		let iter = map.as_slice().iter();
		Self { iter }
	}

	/// Gets a slice of the key-value pairs.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_slice(&self) -> &[(K, V)] {
		self.iter.as_slice()
	}
}

impl<K, V> Debug for Iter<'_, K, V>
where
	K: Debug,
	V: Debug,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("Iter")
			.field(&self.iter.as_slice())
			.finish()
	}
}

impl<K, V> Default for Iter<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back()
	}
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> { }

impl<K, V> FusedIterator for Iter<'_, K, V> { }

impl<'a, K, V> Iterator for Iter<'a, K, V> {
	type Item = &'a (K, V);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
