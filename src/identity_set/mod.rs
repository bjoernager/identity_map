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

//! The [`IdentitySet`] and associated facilities.

use crate::use_mod;

use_mod!(pub difference);
use_mod!(pub identity_set);
use_mod!(pub intersection);
use_mod!(pub into_iter);
use_mod!(pub iter);
use_mod!(pub symmetric_difference);
use_mod!(pub union);

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
