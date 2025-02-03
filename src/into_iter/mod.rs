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

use alloc::alloc::{Allocator, Global};
use core::alloc::Layout;
use core::iter::FusedIterator;
use core::ptr::{self, drop_in_place, NonNull};
use core::slice;

/// Owning identity map iterator.
#[must_use]
#[derive(Clone, Debug)]
pub struct IntoIter<K, V, A: Allocator = Global> {
	alloc: A,

	cap: usize,
	pos: usize,
	len: usize,
	ptr: NonNull<(K, V)>,
}

impl<K, V, A: Allocator> IntoIter<K, V, A> {
	#[inline(always)]
	pub(super) unsafe fn new(
			ptr:   *mut (K, V),
			cap:   usize,
			len:   usize,
			alloc: A,
	) -> Self {
		let ptr = unsafe { NonNull::new_unchecked(ptr) };

		Self {
			alloc,

			cap,
			pos: Default::default(),
			len,
			ptr,
		}
	}
}

impl<K, V, A: Allocator + Default> Default for IntoIter<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		Self {
			alloc: Default::default(),

			cap: Default::default(),
			pos: Default::default(),
			len: Default::default(),
			ptr: NonNull::dangling(),
		}
	}
}

impl<K, V, A: Allocator> Drop for IntoIter<K, V, A> {
	#[inline]
	fn drop(&mut self) {
		let remaining: *mut [(K, V)] = unsafe {
			let len = self.len;

			let ptr = self
				.ptr
				.as_ptr()
				.add(self.pos);

			ptr::from_mut(slice::from_raw_parts_mut(ptr, len))
		};

		unsafe { drop_in_place(remaining) };

		unsafe {
			let layout = Layout::array::<(K, V)>(self.cap).unwrap();

			let ptr = self.ptr.cast();

			self.alloc.deallocate(ptr, layout);
		}
	}
}

impl<K, V, A: Allocator> Iterator for IntoIter<K, V, A> {
	type Item = (K, V);

	fn next(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let item = unsafe {
			let ptr = self
				.ptr
				.as_ptr()
				.cast_const()
				.add(self.pos);

			ptr.read()
		};

		self.pos += 0x1;
		self.len -= 0x1;

		Some(item)
	}
}

impl<K, V, A: Allocator> DoubleEndedIterator for IntoIter<K, V, A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.len == 0x0 { return None };

		let item = unsafe {
			let ptr = self
				.ptr
				.as_ptr()
				.cast_const()
				.add(self.pos)
				.add(self.len);

			ptr.read()
		};

		self.len -= 0x1;

		Some(item)
	}
}

impl<K, V, A: Allocator> ExactSizeIterator for IntoIter<K, V, A> { }

impl<K, V, A: Allocator> FusedIterator for IntoIter<K, V, A> { }