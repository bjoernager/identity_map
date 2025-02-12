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

#[cfg(test)]
mod test;

use crate::identity_map::IdentityMap;
use crate::identity_set::{IntoIter, Iter, IterMut};

use allocator_api2::alloc::{Allocator, Global};
use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::mem::ManuallyDrop;

/// An ordered identity set.
///
/// This set records a list of keys wherein each key is unique.
///
/// Unlike other sets such as [`HashSet`](std::collections::HashSet), this type only transforms keys as if the [`identity`](core::convert::identity) function was used.
#[repr(transparent)]
#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct IdentitySet<T, A: Allocator = Global> {
	map: IdentityMap<T, (), A>,
}

impl<T> IdentitySet<T> {
	/// Constructs a new, empty identity set.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const fn new() -> Self {
		Self::new_in(Global)
	}

	/// Preallocates a new identity set.
	///
	/// # Panics
	///
	/// If `[T; cap]` could not be allocated using the global allocator, then this function will panic.
	///
	/// This function will also panic if `cap` is greater than [`isize::MAX`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity(cap: usize) -> Self {
		Self::with_capacity_in(cap, Global)
	}

	/// Constructs a new identity set from raw parts.
	///
	/// # Safety
	///
	/// See [`IdentityMap::from_raw_parts`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts(ptr: *mut T, cap: usize, len: usize) -> Self {
		unsafe { Self::from_raw_parts_in(ptr, cap, len, Global) }
	}
}

impl<T, A: Allocator> IdentitySet<T, A> {
	/// Constructs a new, empty identity set with a specific allocator.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const fn new_in(alloc: A) -> Self {
		let map = IdentityMap::new_in(alloc);
		Self { map }
	}

	/// Preallocates a new identity set with a specific allocator.
	///
	/// # Panics
	///
	/// If `[T; cap]` could not be allocated with the given allocator, then this function will panic.
	///
	/// This function will also panic if `cap` is greater than [`isize::MAX`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity_in(cap: usize, alloc: A) -> Self {
		let map = IdentityMap::with_capacity_in(cap, alloc);
		Self { map }
	}

	/// Constructs a new identity set from raw parts.
	///
	/// # Safety
	///
	/// See [`IdentityMap::from_raw_parts_in`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts_in(
		ptr:   *mut T,
		cap:   usize,
		len:   usize,
		alloc: A,
	) -> Self {
		// SAFETY: `(T, ())` is transparent to `T`.
		let ptr = ptr as *mut (T, ());

		// SAFETY: Caller guarantees the validity of the
		// parts.
		let map = unsafe { IdentityMap::from_raw_parts_in(ptr, cap, len, alloc) };

		Self { map }
	}

	/// Reserves additional capacity for the set.
	///
	/// # Panics
	///
	/// This method will panic if the internal buffer could not be grown.
	/// It will also panic if the new capacity of the set is greater than [`isize::MAX`].
	#[inline(always)]
	#[track_caller]
	pub fn reserve(&mut self, count: usize) {
		self.map.reserve(count);
	}

	/// Borrows the set's allocator.
	#[inline(always)]
	#[must_use]
	pub fn allocator(&self) -> &A {
		self.map.allocator()
	}

	/// Gets a iterator of the containedf key-value pairs.
	#[inline(always)]
	pub fn iter(&self) -> Iter<T> {
		Iter::new(self)
	}

	/// Gets a mutable iterator of the contained key-value pairs.
	#[inline(always)]
	pub fn iter_mut(&mut self) -> IterMut<T> {
		IterMut::new(self)
	}

	/// Retrieves the total capacity of the set.
	///
	/// Remember that this capacity can -- if needed to -- be increased using the [`reserve`](Self::reserve) method.
	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.map.capacity()
	}

	/// Retrieves the current length of the set.
	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.map.len()
	}

	/// Tests if the set is empty.
	#[inline(always)]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	/// Gets a pointer to the set buffer.
	///
	/// Note that this pointer may necessarily be dangling if the set isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const T {
		// SAFETY: `(T, ())` is transparent to `T`.
		self.map.as_ptr() as *const T
	}

	/// Gets a mutable pointer to the set buffer.
	///
	/// Note that this pointer may necessarily be dangling if the set isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		// SAFETY: `(T, ())` is transparent to `T`.
		self.map.as_mut_ptr() as *mut T
	}

	/// Gets a slice over the set's key-value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[T] {
		// SAFETY: `(T, ())` is transparent to `T`.
		unsafe { &*(&raw const *self.map.as_slice() as *const [T]) }
	}

	/// Gets a mutable slice over the set's key-value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		// SAFETY: `(T, ())` is transparent to `T`.
		unsafe { &mut *(&raw mut *self.map.as_mut_slice() as *mut [T]) }
	}

	/// Borrows the set as a map.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_map(&self) -> &IdentityMap<T, (), A> {
		&self.map
	}

	/// Mutably borrows the set as a map.
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_mut_map(&mut self) -> &mut IdentityMap<T, (), A> {
		&mut self.map
	}

	/// Converts the set into a map.
	#[inline(always)]
	#[must_use]
	pub(crate) fn into_map(self) -> IdentityMap<T, (), A> {
		self.map
	}
}

impl<T, A> IdentitySet<T, A>
where
	T: Eq + Ord,
	A: Allocator,
{
	/// Inserts a new key-value pair into the set.
	///
	/// If the provided key already exists in the set, then its associated value is simply updated.
	/// The previous value is in that case returned from this method.
	///
	/// # Panics
	///
	/// If the set did not already hold `key` as a key and could not grow its buffer to accommodate the `key` & `value` pair, then this method will panic.
	#[inline(always)]
	#[track_caller]
	pub fn insert(&mut self, key: T) -> bool {
		self.map.insert(key, ()).is_some()
	}

	/// Removes the whole pair associated with the specific key.
	///
	/// The associated value is returned from this method.
	/// If no pair existed with the provided key, then this method will instead return a [`None`] instance.
	#[inline(always)]
	#[track_caller]
	pub fn remove(&mut self, key: &T) -> bool {
		self.map.remove(key).is_some()
	}

	/// Checks if the set contains the specified key.
	#[inline(always)]
	#[must_use]
	pub fn contains<U>(&self, key: &U) -> bool
	where
		T: Borrow<U>,
		U: Eq + Ord + ?Sized,
	{
		self.map.contains_key(key)
	}
}

impl<T, A> Debug for IdentitySet<T, A>
where
	T: Debug,
	A: Allocator,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl<T: Ord, const N: usize> From<[T; N]> for IdentitySet<T> {
	#[inline(always)]
	fn from(value: [T; N]) -> Self {
		let value = ManuallyDrop::new(value);

		// SAFETY: `(T, ())` is transparent to `T`. The
		// previous `value` is also not used at all after
		// this transmutation.
		let value = unsafe { (&raw const value).cast::<[(T, ()); N]>().read() };

		let map = value.into();
		Self { map }
	}
}

impl<T, A: Allocator> IntoIterator for IdentitySet<T, A> {
	type Item = T;

	type IntoIter = IntoIter<T, A>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(self)
	}
}

impl<'a, T, A: Allocator> IntoIterator for &'a IdentitySet<T, A> {
	type Item = &'a T;

	type IntoIter = Iter<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, T, A: Allocator> IntoIterator for &'a mut IdentitySet<T, A> {
	type Item = &'a mut T;

	type IntoIter = IterMut<'a, T>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
