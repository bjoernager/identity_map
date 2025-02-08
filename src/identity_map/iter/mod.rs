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

use crate::identity_map::{IdentityMap, RawIter};

use allocator_api2::alloc::Allocator;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem::transmute;
use core::ptr;

/// Borrowing identity map iterator.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Default)]
pub struct Iter<'a, K, V> {
	raw: RawIter<K, V>,

	_buf: PhantomData<&'a [(K, V)]>,
}

impl<'a, K, V> Iter<'a, K, V> {
	/// Constructs a new, borrowing identity map iterator.
	#[inline(always)]
	pub(crate) fn new<A: Allocator>(map: &IdentityMap<K, V, A>) -> Self {
		let buf = ptr::from_ref(map.as_slice());

		// SAFETY: References are always non-null and ini-
		// tialised at their destination.
		let raw = unsafe { RawIter::new(buf.cast_mut()) };

		Self { raw, _buf: PhantomData, }
	}

	/// Gets a pointer to the first key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const (K, V) {
		self.raw.as_ptr()
	}

	/// Gets a slice of the key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &'a [(K, V)] {
		// SAFETY: We guarantee that the lifetime is con-
		// tained.
		unsafe { &*self.raw.as_slice() }
	}
}

impl<K, V> Debug for Iter<'_, K, V>
where
	K: Debug,
	V: Debug,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f
			.debug_tuple("Iter")
			.field(&self.as_slice())
			.finish()
	}
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
	#[inline(always)]
	fn next_back(&mut self) -> Option<Self::Item> {
		// SAFETY: We guarantee that items are ini-
		// tialised, and `Option<&(K, V)>` is transparent
		// to `Option<*const (K, V)>`.
		unsafe { transmute(self.raw.next_back()) }
	}
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> { }

impl<K, V> FusedIterator for Iter<'_, K, V> { }

impl<'a, K, V> Iterator for Iter<'a, K, V> {
	type Item = &'a (K, V);

	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		// SAFETY: We guarantee that items are ini-
		// tialised, and `Option<&(K, V)>` is transparent
		// to `Option<*const (K, V)>`.
		unsafe { transmute(self.raw.next()) }
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.raw.size_hint()
	}
}
// SAFETY: `Sync` guarantees that the type can also
// be sent.
unsafe impl<K, V> Send for Iter<'_, K, V>
where
	K: Sync,
	V: Sync,
{ }

unsafe impl<K, V> Sync for Iter<'_, K, V>
where
	K: Sync,
	V: Sync,
{ }
