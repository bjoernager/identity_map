// Copyright 2025 Gabriel Bjørnager Jensen.

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
