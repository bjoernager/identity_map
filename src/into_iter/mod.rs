// Copyright (c) 2025 Gabriel Bjørnager Jensen.
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
// CHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE
// AND NONINFRINGEMENT. IN NO EVENT SHALL THE AU-
// THORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::RawIdentityMap;

use core::fmt::{self, Debug, Formatter};
use alloc::alloc::{Allocator, Global};
use core::iter::FusedIterator;
use core::ptr::{self, drop_in_place};

/// Owning identity map iterator.
#[must_use]
#[derive(Clone, Default)]
pub struct IntoIter<K, V, A: Allocator = Global> {
	pos: usize,
	raw: RawIdentityMap<K, V, A>,
}

impl<K, V, A: Allocator> IntoIter<K, V, A> {
	/// Constructs a new, owning identity map iterator.
	///
	/// # Safety
	///
	/// The provided, raw identity map must be initialised.
	#[inline(always)]
	pub(super) unsafe fn new(raw: RawIdentityMap<K, V, A>) -> Self {
		Self {
			pos: Default::default(),
			raw,
		}
	}

	/// Gets a pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const (K, V) {
		unsafe { self.raw.as_ptr().add(self.pos) }
	}

	/// Gets a mutable pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		unsafe { self.raw.as_mut_ptr().add(self.pos) }
	}

	/// Gets a slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[(K, V)] {
		unsafe { &*self.raw.as_slice() }
	}

	/// Gets a mutable slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
		unsafe { &mut *self.raw.as_mut_slice() }
	}
}

impl<K, V, A: Allocator> AsMut<[(K, V)]> for IntoIter<K, V, A> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [(K, V)] {
		self.as_mut_slice()
	}
}

impl<K, V, A: Allocator> AsRef<[(K, V)]> for IntoIter<K, V, A> {
	#[inline(always)]
	fn as_ref(&self) -> &[(K, V)] {
		self.as_slice()
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
			.debug_tuple("IterMut")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, V, A: Allocator> Drop for IntoIter<K, V, A> {
	#[inline(always)]
	fn drop(&mut self) {
		// Drop all items that are still alive.

		let remaining = ptr::from_mut(self.as_mut_slice());
		unsafe { drop_in_place(remaining) };
	}
}

impl<K, V, A: Allocator> Iterator for IntoIter<K, V, A> {
	type Item = (K, V);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let mut len = self.len();

		if len == 0x0 { return None };

		let item = unsafe {
			let ptr = self
				.raw
				.as_ptr()
				.add(self.pos);

			ptr.read()
		};

		self.pos += 0x1;
		len      -= 0x1;

		unsafe { self.raw.set_len(len) };

		Some(item)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.raw.len();
		(size, Some(size))
	}
}

impl<K, V, A: Allocator> DoubleEndedIterator for IntoIter<K, V, A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		let mut len = self.len();

		if len == 0x0 { return None };

		let item = unsafe {
			let ptr = self
				.raw
				.as_ptr()
				.add(self.pos)
				.add(len);

			ptr.read()
		};

		len -= 0x1;
		unsafe { self.raw.set_len(len) };

		Some(item)
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoIter<K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for IntoIter<K, V, A> { }