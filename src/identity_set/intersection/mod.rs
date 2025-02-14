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

use crate::identity_set::{IdentitySet, Iter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Iterator denoting the [intersection](https://en.wikipedia.org/wiki/Intersection) between two [identity sets](IdentitySet).
#[must_use]
#[derive(Clone)]
pub struct Intersection<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	this:  Iter<'a, T>,
	other: &'a IdentitySet<T, A>,
}

impl<'a, T, A: Allocator> Intersection<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	/// Constructs a new iterator denoting the [intersection](https://en.wikipedia.org/wiki/Intersection) between two [identity sets](IdentitySet).
	#[inline(always)]
	pub(crate) fn new(this: &'a IdentitySet<T, A>, other: &'a IdentitySet<T, A>) -> Self {
		let this = this.iter();
		Self { this, other }
	}
}

impl<T, A: Allocator> FusedIterator for Intersection<'_, T, A>
where
	T: Ord,
	A: Allocator,
{ }

impl<'a, T, A> Iterator for Intersection<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	type Item = &'a T;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		for key in self.this.by_ref() {
			if self.other.contains(key) { return Some(key) };
		}

		None
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		// Either the sets do not overlap at all or they
		// are subsets of each other.

		let min = 0x0;
		let max = self.this.len().min(self.other.len());

		(min, Some(max))
	}
}
