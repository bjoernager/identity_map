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

use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::vec;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;

/// Identity map drain.
#[must_use]
pub struct Drain<'a, K, V, A: Allocator = Global> {
	iter: vec::Drain<'a, (K, V), A>,
}

impl<'a, K, V, A: Allocator> Drain<'a, K, V, A> {
	/// Constructs a new identity map drain.
	#[inline(always)]
	pub(crate) fn new(map: &'a mut IdentityMap<K, V, A>) -> Self {
		let iter = map.as_mut_vec().drain(..);
		Self { iter }
	}

	/// Gets a slice of the key-value pairs.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_slice(&self) -> &[(K, V)] {
		self.iter.as_slice()
	}
}

impl<K, V, A> Debug for Drain<'_, K, V, A>
where
	K: Debug,
	V: Debug,
	A: Allocator,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("Drain")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, V, A: Allocator> Iterator for Drain<'_, K, V, A> {
	type Item = (K, V);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter.size_hint()
	}
}

impl<K, V, A: Allocator> DoubleEndedIterator for Drain<'_, K, V, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back()
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for Drain<'_, K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for Drain<'_, K, V, A> { }
