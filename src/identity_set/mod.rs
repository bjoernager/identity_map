// Copyright 2025 Gabriel Bjørnager Jensen.

//! The [`IdentitySet`] type and associated facilities.

mod difference;
mod drain;
mod identity_set;
mod intersection;
mod into_iter;
mod iter;
mod symmetric_difference;
mod union;

pub use difference::Difference;
pub use drain::Drain;
pub use identity_set::IdentitySet;
pub use intersection::Intersection;
pub use into_iter::IntoIter;
pub use iter::Iter;
pub use symmetric_difference::SymmetricDifference;
pub use union::Union;

use core::cmp::Ordering;
use core::iter::Peekable;

/// Picks next. smallest value between two iterator.
///
/// Note that both iterators must already themselves be sorted.
#[inline]
#[must_use]
fn next_sorted<I, J>(liter: &mut Peekable<I>, riter: &mut Peekable<J>) -> Option<I::Item>
where
	I: Iterator<Item: Ord>,
	J: Iterator<Item = I::Item>,
{
	let lhs = liter.peek();
	let rhs = riter.peek();

	// Select the largest key between the two iterator
	// and continue on the appropriate iterator.

	match (lhs, rhs) {
		(None,    None)    => None,
		(Some(_), None)    => Some(liter.next().unwrap()),
		(None,    Some(_)) => Some(riter.next().unwrap()),

		(Some(lhs), Some(rhs)) => {
			match lhs.cmp(rhs) {
				Ordering::Equal => {
					// The left hand size takes precedence.

					let _ = riter.next();

					Some(liter.next().unwrap())
				}

				Ordering::Less    => Some(liter.next().unwrap()),
				Ordering::Greater => Some(riter.next().unwrap()),
			}
		}
	}
}
