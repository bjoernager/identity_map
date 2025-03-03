// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_set::{Difference, IdentitySet, next_sorted};

use allocator_api2::alloc::Allocator;
use core::iter::{FusedIterator, Peekable};

/// Iterator denoting the [symmetric difference](https://en.wikipedia.org/wiki/Symmetric_difference) between two [identity sets](IdentitySet).
#[must_use]
#[derive(Clone)]
pub struct SymmetricDifference<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	lhs: Peekable<Difference<'a, T, A>>,
	rhs: Peekable<Difference<'a, T, A>>,
}

impl<'a, T, A: Allocator> SymmetricDifference<'a, T, A>
where
	T: Ord,
	A: Allocator,
{
	/// Constructs a new iterator denoting the [symmetric difference](https://en.wikipedia.org/wiki/Symmetric_difference) between two [identity sets](IdentitySet).
	#[inline(always)]
	pub(crate) fn new(this: &'a IdentitySet<T, A>, other: &'a IdentitySet<T, A>) -> Self {
		let lhs = this.difference(other).peekable();
		let rhs = other.difference(this).peekable();

		Self { lhs, rhs }
	}
}

impl<T, A: Allocator> FusedIterator for SymmetricDifference<'_, T, A>
where
	T: Ord,
	A: Allocator,
{ }

impl<'a, T, A> Iterator for SymmetricDifference<'a, T, A>
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
		// Either boths sets are identical or they do not
		// overlap at all.

		let min = 0x0;
		let max = self.lhs.size_hint().1.unwrap() + self.rhs.size_hint().1.unwrap();

		(min, Some(max))
	}
}
