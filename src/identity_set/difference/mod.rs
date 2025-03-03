// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_set::{IdentitySet, Iter};

use allocator_api2::alloc::Allocator;
use core::iter::FusedIterator;

/// Iterator denoting the [difference](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement) between two [identity sets](IdentitySet).
#[must_use]
#[derive(Clone)]
pub struct Difference<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	this:  Iter<'a, T>,
	other: &'a IdentitySet<T, A>,
}

impl<'a, T, A: Allocator> Difference<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	/// Constructs a new iterator denoting the [difference](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement) between two [identity sets](IdentitySet).
	#[inline(always)]
	pub(crate) fn new(this: &'a IdentitySet<T, A>, other: &'a IdentitySet<T, A>) -> Self {
		let this = this.iter();
		Self { this, other }
	}
}

impl<T, A: Allocator> FusedIterator for Difference<'_, T, A>
where
	T: Ord,
	A: Allocator,
{ }

impl<'a, T, A> Iterator for Difference<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	type Item = &'a T;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		for key in self.this.by_ref() {
			if !self.other.contains(key) { return Some(key) };
		}

		None
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		// Either boths sets are identical or they do not
		// overlap at all.

		let min = 0x0;
		let max = self.this.len();

		(min, Some(max))
	}
}
