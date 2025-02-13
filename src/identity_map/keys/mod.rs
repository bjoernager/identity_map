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

use crate::identity_map::{IdentityMap, Iter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Borrowing identity map keys iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Keys<'a, K, V> {
	iter: Iter<'a, K, V>,
}

impl<'a, K, V> Keys<'a, K, V> {
	/// Constructs a new, borrowing identity map keys iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a IdentityMap<K, V, A>) -> Self {
		let iter = map.iter();
		Self { iter }
	}
}

impl<K, V> Default for Keys<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for Keys<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(k, _)| k)
	}
}

impl<K, V> ExactSizeIterator for Keys<'_, K, V> { }

impl<K, V> FusedIterator for Keys<'_, K, V> { }

impl<'a, K, V> Iterator for Keys<'a, K, V> {
	type Item = &'a K;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(k, _)| k)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
