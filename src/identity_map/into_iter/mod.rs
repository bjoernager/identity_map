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
use allocator_api2::vec::{self, Vec};
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;

 /// Owning identity map iterator.
#[must_use]
#[derive(Clone)]
pub struct IntoIter<K, V, A: Allocator = Global> {
	iter: vec::IntoIter<(K, V), A>,
}

impl<K, V, A: Allocator> IntoIter<K, V, A> {
	/// Constructs a new, owning identity map iterator.
	///
	/// # Safety
	///
	/// The provided, raw identity map must be initialised.
	#[inline(always)]
	pub(crate) fn new(map: IdentityMap<K, V, A>) -> Self {
		let iter = map.into_vec().into_iter();
		Self { iter }
	}

	/// Gets a slice of the key-value pairs.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_slice(&self) -> &[(K, V)] {
		self.iter.as_slice()
	}
}

impl<K, V, A> Debug for IntoIter<K, V, A>
where
	K: Debug,
	V: Debug,
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

impl<K, V, A: Allocator + Default> Default for IntoIter<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		let alloc = Default::default();

		let iter = Vec::new_in(alloc).into_iter();
		Self { iter }
	}
}

impl<K, V, A: Allocator> Iterator for IntoIter<K, V, A> {
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

impl<K, V, A: Allocator> DoubleEndedIterator for IntoIter<K, V, A> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.iter.next_back()
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoIter<K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for IntoIter<K, V, A> { }
