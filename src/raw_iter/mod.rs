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

use core::iter::FusedIterator;
use core::ptr::{self, NonNull};

#[must_use]
#[derive(Clone)]
pub struct RawIter<K, V> {
	pos: usize,
	len: usize,
	ptr: NonNull<(K, V)>,
}

impl<K, V> RawIter<K, V> {
	#[inline(always)]
	pub unsafe fn new(buf: *mut [(K, V)]) -> Self {
		debug_assert!(!buf.is_null());

		// SAFETY: Caller guarnatees the validity of `buf`.
		let ptr = unsafe { NonNull::new_unchecked(buf.cast()) };

		Self {
			len: buf.len(),
			ptr,

			..Default::default()
		}
	}

	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const (K, V) {
		self.ptr.as_ptr().cast_const()
	}

	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		self.ptr.as_ptr()
	}

	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> *const [(K, V)] {
		let len = self.len;

		// SAFETY: `pos` will always be within bounds.
		let ptr = unsafe { self.ptr.as_ptr().cast_const().add(self.pos) };

		ptr::slice_from_raw_parts(ptr, len)
	}

	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> *mut [(K, V)] {
		let len = self.len;

		// SAFETY: `pos` will always be within bounds.
		let ptr = unsafe { self.ptr.as_ptr().add(self.pos) };

		ptr::slice_from_raw_parts_mut(ptr, len)
	}
}

impl<K, V> Default for RawIter<K, V> {
	#[inline(always)]
	fn default() -> Self {
		Self {
			ptr: NonNull::dangling(),
			pos: Default::default(),
			len: Default::default(),
		}
	}
}

impl<K, V> Iterator for RawIter<K, V> {
	type Item = NonNull<(K, V)>;

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let item = unsafe { self.ptr.add(self.pos) };

		self.pos += 0x1;
		self.len -= 0x1;

		Some(item)
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.len;
		(size, Some(size))
	}
}

impl<K, V> DoubleEndedIterator for RawIter<K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let item = unsafe { self.ptr.add(self.pos).add(self.len) };

		self.len -= 0x1;

		Some(item)
	}
}

impl<K, V> ExactSizeIterator for RawIter<K, V> { }

impl<K, V> FusedIterator for RawIter<K, V> { }
