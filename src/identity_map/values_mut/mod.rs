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

use crate::identity_map::{IdentityMap, IterMut};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Mutably-borrowing identity map values iterator.
#[must_use]
#[repr(transparent)]
#[derive(Debug)]
pub struct ValuesMut<'a, K, V> {
	iter: IterMut<'a, K, V>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
	/// Constructs a new, mutably-borrowing identity map values iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &'a mut IdentityMap<K, V, A>) -> Self {
		let iter = map.iter_mut();
		Self { iter }
	}
}

impl<K, V> Default for ValuesMut<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		let iter = Default::default();
		Self { iter }
	}
}

impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back().map(|(_, v)| v)
	}
}

impl<K, V> ExactSizeIterator for ValuesMut<'_, K, V> { }

impl<K, V> FusedIterator for ValuesMut<'_, K, V> { }

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
	type Item = &'a mut V;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|(_, v)| v)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}
