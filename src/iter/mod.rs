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
//  ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// Borrowing identity map iterator.
#[must_use]
#[derive(Clone, Debug)]
pub struct Iter<'a, K, V> {
	pos: usize,
	len: usize,
	ptr: NonNull<(K, V)>,

	_buf: PhantomData<&'a [(K, V)]>,
}

impl<'a, K, V> Iter<'a, K, V> {
	#[inline(always)]
	pub(super) fn new(buf: &'a [(K, V)]) -> Self {
		let len = buf.len();

		let ptr = unsafe {
			let ptr = buf.as_ptr().cast_mut();

			NonNull::new_unchecked(ptr)
		};

		Self {
			len,
			ptr,

			..Default::default()
		}
	}
}

impl<K, V> Default for Iter<'_, K, V> {
	#[inline(always)]
	fn default() -> Self {
		Self {
			ptr: NonNull::dangling(),

			pos: Default::default(),
			len: Default::default(),

			_buf: Default::default(),
		}
	}
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
	type Item = &'a (K, V);

	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let item = unsafe {
			let ptr = self
				.ptr
				.as_ptr()
				.cast_const()
				.add(self.pos);

			&*ptr
		};

		self.pos += 0x1;
		self.len -= 0x1;

		Some(item)
	}
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let item = unsafe {
			let ptr = self
				.ptr
				.as_ptr()
				.cast_const()
				.add(self.pos)
				.add(self.len);

			&*ptr
		};

		self.len -= 0x1;

		Some(item)
	}
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> { }

impl<K, V> FusedIterator for Iter<'_, K, V> { }