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

use alloc::alloc::{Allocator, Global};
use core::alloc::Layout;
use core::any::type_name;
use core::mem::ManuallyDrop;
use core::ptr::{self, copy_nonoverlapping, NonNull};

// NOTE: `cap` can always safely be cast to `usize`
// if `is_allocated` returns `false`. This is also
// the preferred option, if possible, over using
// `capacity`.

pub struct RawIdentityMap<K, V, A: Allocator = Global> {
	alloc: A,

	cap: isize,
	len: usize,
	ptr: NonNull<(K, V)>,
}

impl<K, V, A: Allocator> RawIdentityMap<K, V, A> {
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn new_in(alloc: A) -> Self {
		Self {
			alloc,

			cap: isize::MIN,
			len: Default::default(),
			ptr: NonNull::dangling(),
		}
	}

	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity_in(count: usize, alloc: A) -> Self {
		let mut this = Self::new_in(alloc);
		this.allocate(count);

		this
	}

	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts_in(
		ptr:   *mut (K, V),
		cap:   usize,
		len:   usize,
		alloc: A,
	) -> Self {
		debug_assert!(cap <= isize::MAX as usize);
		let cap = cap as isize;

		let ptr = match cap {
			0x0 => NonNull::dangling(),
			_   => unsafe { NonNull::new_unchecked(ptr) }
		};

		Self {
			alloc,

			cap,
			len,
			ptr,
		}
	}

	#[inline]
	#[track_caller]
	fn allocate(&mut self, count: usize) {
		let layout = match Layout::array::<(K, V)>(count) {
			Ok(layout) => layout,

			Err(e) => {
				let type_name = type_name::<(K, V)>();

				panic!("unable to create layout for `[{type_name}; {count}]`: {e}");
			}
		};

		// Note: `Layout::array` tests that `count` is not
		// greater than `isize::MAX`.

		let ptr = match self.alloc.allocate(layout) {
			Ok(ptr) => ptr,

			Err(e) => panic!("unable to allocate: {e}"),
		};

		self.cap = count as isize;
		self.len = Default::default();
		self.ptr = ptr.cast();
	}

	#[inline]
	#[track_caller]
	pub fn reserve(&mut self, count: usize) {
		// Do not grow if not already allocated.

		if self.is_allocated() {
			// Allocate new buffer.

			self.allocate(count);
			return;
		}

		// Grow existing buffer.

		let old_cap = self.capacity();
		let new_cap = self.capacity() + count;

		let old_layout = Layout::array::<(K, V)>(old_cap).unwrap();

		let new_layout = match Layout::array::<(K, V)>(new_cap) {
			Ok(layout) => layout,

			Err(e) => {
				let type_name = type_name::<(K, V)>();

				panic!("unable to create layout for `[{type_name}; {new_cap}]`: {e}");
			}
		};

		// Note: `Layout::array` tests that `new_cap` is not
		// greater than `isize::MAX`.

		let ptr = self.ptr.cast();

		// SAFETY: We guarantee that the following is true:
		//
		// * That `ptr` was previously returned by a call
		//   to `A::allocate`;
		//
		// * That `old_layout` was the layout used in the
		//   initial call to `allocate`;
		let ptr = match unsafe { self.alloc.grow(ptr, old_layout, new_layout) } {
			Ok(ptr) => ptr,

			Err(e) => panic!("unable to allocate: {e}"),
		};

		debug_assert!(ptr.len() <= isize::MAX as usize);

		self.cap = new_cap as isize;
		self.ptr = ptr.cast();
	}

	#[inline(always)]
	#[track_caller]
	pub unsafe fn set_len(&mut self, len: usize) {
		debug_assert!(len <= self.capacity());

		self.len = len;
	}

	#[inline(always)]
	#[must_use]
	pub fn allocator(&self) -> &A {
		self.alloc.by_ref()
	}

	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		let mask = !(isize::MIN as usize);

		(self.cap as usize) & mask
	}

	#[allow(clippy::len_without_is_empty)]
	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.len
	}

	#[inline(always)]
	#[must_use]
	pub fn is_allocated(&self) -> bool {
		self.cap == isize::MIN
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
		let len = self.len();
		let ptr = self.as_ptr();

		ptr::slice_from_raw_parts(ptr, len)
	}

	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> *mut [(K, V)] {
		let len = self.len();
		let ptr = self.as_mut_ptr();

		ptr::slice_from_raw_parts_mut(ptr, len)
	}

	#[inline(always)]
	#[must_use]
	pub fn into_raw_parts_with_allow(mut self) -> (*mut (K, V), usize, usize, A) {
		let cap = self.capacity();
		let len = self.len();
		let ptr = self.as_mut_ptr();

		// Extract the allocator. Remember that we cannot
		// simply take is as `Self` implements `Drop`.

		let this = ManuallyDrop::new(self);

		let alloc = unsafe {
			let ptr = &raw const this.alloc;

			// SAFETY: The original memory is not dropped using
			// `Drop`, so we do not need to worry about `!Copy`
			// types.
			ptr.read()
		};

		(ptr, cap, len, alloc)
	}
}

impl<K, V, A> Clone for RawIdentityMap<K, V, A>
where
	K: Clone,
	V: Clone,
	A: Allocator + Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		let alloc = self.alloc.clone();

		if self.is_allocated() { return Self::new_in(alloc) };

		let cap = self.cap as usize;
		let mut new = Self::with_capacity_in(cap, alloc);

		unsafe {
			let src = self.as_ptr();
			let dst = new.as_mut_ptr();

			copy_nonoverlapping(src, dst, cap);
		}

		new
	}
}

impl<K, V, A: Allocator + Default> Default for RawIdentityMap<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		Self::new_in(Default::default())
	}
}

impl<K, V, A: Allocator> Drop for RawIdentityMap<K, V, A> {
	#[inline]
	fn drop(&mut self) {
		// Do not deallocate if unallocated.

		if self.is_allocated() { return };

		// Do not drop any items as we do not guarantee any
		// of them being initialised.

		// Deallocate the buffer.

		unsafe {
			let layout = Layout::array::<(K, V)>(self.cap as usize).unwrap();

			let ptr = self.ptr.cast();

			self.alloc.deallocate(ptr, layout);
		}
	}
}

unsafe impl<K, V, A> Send for RawIdentityMap<K, V, A>
where
	K: Send,
	V: Send,
	A: Allocator  + Send,
{ }

unsafe impl<K, V, A> Sync for RawIdentityMap<K, V, A>
where
	K: Sync,
	V: Sync,
	A: Allocator  + Sync,
{ }
