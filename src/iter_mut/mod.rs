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

use crate::RawIter;

use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem::transmute;
use core::ptr;

/// Mutably-borrowing identity map iterator.
#[must_use]
#[derive(Default)]
pub struct IterMut<'a, K, V> {
	raw: RawIter<K, V>,

	_buf: PhantomData<&'a mut [(K, V)]>,
}

impl<'a, K, V> IterMut<'a, K, V> {
	/// Constructs a new, mutably-borrowing identity map iterator.
	#[inline(always)]
	pub(super) fn new(buf: &'a mut [(K, V)]) -> Self {
		let buf = ptr::from_mut(buf);

		// SAFETY: Mutable references are always unique and
		// initialised at their destination.
		let raw = unsafe { RawIter::new(buf) };

		Self { raw, _buf: PhantomData, }
	}

	/// Gets a pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const (K, V) {
		self.raw.as_ptr()
	}

	/// Gets a mutable pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		self.raw.as_mut_ptr()
	}

	/// Gets a slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[(K, V)] {
		// SAFETY: We do guarantee that elements are ini-
		// tialised.
		unsafe { &*self.raw.as_slice() }
	}

	/// Gets a mutable slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
		// SAFETY: We do guarantee that elements are ini-
		// tialised. The mutable `self` reference also
		// guarantees exclusivity.
		unsafe { &mut *self.raw.as_mut_slice() }
	}
}

impl<K, V> AsMut<[(K, V)]> for IterMut<'_, K, V> {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [(K, V)] {
		self.as_mut_slice()
	}
}

impl<K, V> AsRef<[(K, V)]> for IterMut<'_, K, V> {
	#[inline(always)]
	fn as_ref(&self) -> &[(K, V)] {
		self.as_slice()
	}
}

impl<K, V> Debug for IterMut<'_, K, V>
where
	K: Debug,
	V: Debug,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("IterMut")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		// SAFETY: We guarantee that items are ini-
		// tialised, and `Option<&mut (K, V)>` is trans-
		// parent to `Option<*mut (K, V)>`.
		unsafe { transmute(self.raw.next_back()) }
	}
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> { }

impl<K, V> FusedIterator for IterMut<'_, K, V> { }

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
	type Item = &'a mut (K, V);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		// SAFETY: We guarantee that items are ini-
		// tialised, and `Option<&mut (K, V)>` is trans-
		// parent to `Option<*mut (K, V)>`.
		unsafe { transmute(self.raw.next()) }
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.raw.size_hint()
	}
}

// SAFETY: The internal pointer is guaranteed to
// be exclusive.
unsafe impl<K, V> Send for IterMut<'_, K, V>
where
	K: Send,
	V: Send,
{ }

// SAFETY: The internal pointer is guaranteed to
// be exclusive.
unsafe impl<K, V> Sync for IterMut<'_, K, V>
where
	K: Sync,
	V: Sync,
{ }
