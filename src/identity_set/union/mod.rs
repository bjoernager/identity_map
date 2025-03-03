// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_set::{Difference, IdentitySet, Iter, next_sorted};

use allocator_api2::alloc::Allocator;
use core::iter::{FusedIterator, Peekable};

/// Iterator denoting the [union](https://en.wikipedia.org/wiki/Union_(set_theory)) between two [identity sets](IdentitySet).
#[must_use]
#[derive(Clone)]
pub struct Union<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	lhs: Peekable<Iter<'a, T>>,
	rhs: Peekable<Difference<'a, T, A>>,
}

impl<'a, T, A: Allocator> Union<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	/// Constructs a new iterator denoting the [union](https://en.wikipedia.org/wiki/Union_(set_theory)) between two [identity sets](IdentitySet).
	#[inline(always)]
	pub(crate) fn new(this: &'a IdentitySet<T, A>, other: &'a IdentitySet<T, A>) -> Self {
		let lhs = this.iter().peekable();
		let rhs = other.difference(this).peekable();

		Self { lhs, rhs }
	}
}

impl<T, A: Allocator> FusedIterator for Union<'_, T, A>
where
	T: Ord,
	A: Allocator,
{ }

impl<'a, T, A> Iterator for Union<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	type Item = &'a T;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		next_sorted(&mut self.lhs, &mut self.rhs)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		// Either both sets are identical or they do not
		// overlap at all.

		let min = self.lhs.len() + self.rhs.size_hint().0;
		let max = self.lhs.len() + self.rhs.size_hint().1.unwrap();

		(min, Some(max))
	}
}
