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

use crate::identity_map::{IdentityMap, IntoIter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Owning identity map values iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct IntoValues<K, V, A: Allocator> {
	iter: IntoIter<K, V, A>,
}

impl<K, V, A: Allocator> IntoValues<K, V, A> {
	/// Constructs a new, owning identity map values iterator.
	#[inline(always)]
	pub(crate) fn new(map: IdentityMap<K, V, A>) -> Self {
		let iter = map.into_iter();
		Self { iter }
	}
}

impl<K, V, A: Allocator + Default> Default for IntoValues<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V, A: Allocator> DoubleEndedIterator for IntoValues<K, V, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(_, v)| v)
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoValues<K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for IntoValues<K, V, A> { }

impl<K, V, A: Allocator> Iterator for IntoValues<K, V, A> {
	type Item = V;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(_, v)| v)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
