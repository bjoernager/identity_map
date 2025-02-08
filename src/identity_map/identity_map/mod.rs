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

use crate::identity_map::{IntoIter, Iter, IterMut,RawIdentityMap};

use allocator_api2::alloc::{Allocator, Global};
use core::hash::{Hash, Hasher};
use core::mem::{swap, ManuallyDrop};
use core::ops::{Index, IndexMut};
use core::ptr::{self, copy_nonoverlapping, drop_in_place};

/// An allocated identity map.
///
/// This map associates specific keys with specific values.
/// Unlike other maps such as [`HashMap`](std::collections::HashMap), this type only transforms keys as if the [`identity`](core::convert::identity) function was used.
#[repr(transparent)]
#[derive(Default)]
pub struct IdentityMap<K, V, A: Allocator = Global> {
	raw: RawIdentityMap<K, V, A>,
}

impl<K, V> IdentityMap<K, V> {
	/// Constructs a new, empty identity map.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn new() -> Self {
		Self::new_in(Global)
	}

	/// Preallocates a new identity map.
	///
	/// # Panics
	///
	/// If `[(K, V); count]` could not be allocated using the global allocator, then this function will panic.
	///
	/// This function will also panic if `count` is greater than [`isize::MAX`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity(count: usize) -> Self {
		Self::with_capacity_in(count, Global)
	}

	/// Constructs a new identity map from raw parts.
	///
	/// # Safety
	///
	/// The provided parts must have been previously been deconstructed from another identity map.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts(ptr: *mut (K, V), cap: usize, len: usize) -> Self {
		// SAFETY: Caller guarantees the validity of the
		// parts.
		unsafe { Self::from_raw_parts_in(ptr, cap, len, Global) }
	}

	/// Deconstructs the map into its raw parts.
	#[inline(always)]
	#[must_use]
	pub fn into_raw_parts(self) -> (*mut (K, V), usize, usize) {
		let (ptr, cap, len, _alloc) = self.into_raw_parts_with_allow();

		(ptr, cap, len)
	}
}

impl<K, V, A: Allocator> IdentityMap<K, V, A> {
	/// Constructs a new, empty identity map with a specific allocator.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn new_in(alloc: A) -> Self {
		let raw = RawIdentityMap::new_in(alloc);
		Self { raw }
	}

	/// Preallocates a new identity map with a specific allocator.
	///
	/// # Panics
	///
	/// If `[(K, V); count]` could not be allocated with the given allocator, then this function will panic.
	///
	/// This function will also panic if `count` is greater than [`isize::MAX`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity_in(count: usize, alloc: A) -> Self {
		let raw = RawIdentityMap::with_capacity_in(count, alloc);
		Self { raw }
	}

	/// Constructs a new identity map from raw parts.
	///
	/// # Safety
	///
	/// The provided parts must have been previously been deconstructed from another identity map.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts_in(
		ptr:   *mut (K, V),
		cap:   usize,
		len:   usize,
		alloc: A,
	) -> Self {
		let raw = unsafe { RawIdentityMap::from_raw_parts_in(ptr, cap, len, alloc) };
		Self { raw }
	}

	/// Reserves additional capacity for the map.
	///
	/// # Panics
	///
	/// This method will panic if the internal buffer could not be grown.
	/// It will also panic if the new capacity of the map is greater than [`isize::MAX`].
	#[inline(always)]
	#[track_caller]
	pub fn reserve(&mut self, count: usize) {
		self.raw.reserve(count);
	}

	/// Borrows the map's allocator.
	#[inline(always)]
	#[must_use]
	pub fn allocator(&self) -> &A {
		self.raw.allocator()
	}

	/// Gets a iterator of the containedf key-value pairs.
	#[inline(always)]
	pub fn iter(&self) -> Iter<K, V> {
		Iter::new(self)
	}

	/// Gets a mutable iterator of the contained key-value pairs.
	#[inline(always)]
	pub fn iter_mut(&mut self) -> IterMut<K, V> {
		IterMut::new(self)
	}

	/// Retrieves the total capacity of the map.
	///
	/// Remember that this capacity can -- if needed to -- be increased using the [`reserve`](Self::reserve) method.
	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.raw.capacity()
	}

	/// Retrieves the current length of the map.
	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.raw.len()
	}

	/// Tests if the map is empty.
	#[inline(always)]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0x0
	}

	/// Gets a pointer to the map buffer.
	///
	/// Note that this pointer may necessarily be dangling if the map isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const (K, V) {
		self.raw.as_ptr()
	}

	/// Gets a mutable pointer to the map buffer.
	///
	/// Note that this pointer may necessarily be dangling if the map isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		self.raw.as_mut_ptr()
	}

	/// Gets a slice over the map's key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[(K, V)] {
		// SAFETY: We guarantee that all items are ini-
		// tialised.
		unsafe { &*self.raw.as_slice() }
	}

	/// Gets a mutable slice over the map's key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
		// SAFETY: We guarantee that all items are ini-
		// tialised.
		unsafe { &mut *self.raw.as_mut_slice() }
	}

	#[inline(always)]
	#[must_use]
	pub(crate) fn into_raw_identity_map(self) -> RawIdentityMap<K, V, A> {
		let this = ManuallyDrop::new(self);

		// SAFETY: `raw` is not used in `this` after this
		// read as `this` does not implement `Drop`.
		unsafe { (&raw const this.raw).read() }
	}

	/// Deconstructs the map into its raw parts.
	#[inline(always)]
	#[must_use]
	pub fn into_raw_parts_with_allow(self) -> (*mut (K, V), usize, usize, A) {
		let raw = self.into_raw_identity_map();
		raw.into_raw_parts_with_allow()
	}
}

impl<K, V, A> IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	/// Inserts a new key-value pair into the map.
	///
	/// If the provided key already exists in the map, then its associated value is simply updated.
	/// The previous value is in that case returned from this method.
	///
	/// # Panics
	///
	/// If the map did not already hold `key` as a key and could not grow its buffer to accommodate the `key` & `value` pair, then this method will panic.
	#[inline]
	#[track_caller]
	pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
		// Check if we already have the key, and if so up-
		// date its value and short-circuit.

		for (other_key, other_value) in self.iter_mut() {
			if *other_key == key {
				swap(other_value, &mut value);

				return Some(value);
			}
		}

		// Reserve room for another element if there isn't
		// already room for it.

		if self.len() == self.capacity() {
			self.reserve(0x1);
		}

		debug_assert!(self.capacity() > self.len());

		// Insert the new pair into the slot.

		let index = self.len();

		// SAFETY: `index` will always be within bounds as
		// we've just reserved an extra item.
		unsafe { self.raw.insert(index, key, value) };

		// Return nothing as the key wasn't already pre-
		// sent.

		None
	}

	/// Removes the whole pair associated with the specific key.
	///
	/// The associated value is returned from this method.
	/// If no pair existed with the provided key, then this method will instead return a [`None`] instance.
	#[inline]
	#[track_caller]
	pub fn remove(&mut self, key: &K) -> Option<V> {
		let mut index = None;

		// Search for the given key.

		for (other_index, (other_key, _)) in self.iter().enumerate() {
			if *other_key == *key {
				index = Some(other_index);
			}
		}

		// Return if it the key wasn't present.

		let index = index?;

		debug_assert!(index <= self.capacity());

		// Retrieve the value from the buffer.

		// SAFETY: `index`
		let (_, value) = unsafe { self.raw.remove(index) };

		Some(value)
	}

	/// Borrows the associated value of a key.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn get(&self, key: &K) -> Option<&V> {
		for (other_key, other_value) in self {
			if *other_key == *key { return Some(other_value) };
		}

		None
	}

	/// Mutably borrows the associated value of a key.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		for (other_key, other_value) in self {
			if *other_key == *key { return Some(other_value) };
		}

		None
	}

	/// Checks if the map contains the specified key.
	#[inline(always)]
	#[must_use]
	pub fn contains(&self, key: &K) -> bool {
		self.get(key).is_some()
	}
}

impl<K, V, A> Clone for IdentityMap<K, V, A>
where
	K: Clone,
	V: Clone,
	A: Allocator + Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		let len = self.len();

		let mut raw = self.raw.clone();

		for i in 0x0..len {
			// SAFETY: `i` is within bounds and the item at
			// that index is initialised.
			let item = unsafe { &*self.raw.as_ptr().add(i) };

			let value = item.clone();

			// SAFETY: `i` is likewise a valid index here.
			let slot = unsafe { raw.as_mut_ptr().add(i) };

			unsafe { slot.write(value) };
		}

		// SAFETY: The first `len` elements have been ini-
		// tialised.
		unsafe { raw.set_len(len) };

		Self { raw }
	}
}

impl<K, V, A: Allocator> Drop for IdentityMap<K, V, A> {
	#[inline(always)]
	fn drop(&mut self) {
		// Drop all items that are still alive.

		let remaining = ptr::from_mut(self.as_mut_slice());

		// SAFETY: `as_mut_slice` guarantees a valid ref-
		// erence to valid objects.
		unsafe { drop_in_place(remaining) };
	}
}

impl<K, V, const N: usize> From<[(K, V); N]> for IdentityMap<K, V> {
	#[inline]
	fn from(value: [(K, V); N]) -> Self {
		let value = ManuallyDrop::new(value);

		let mut this = Self::with_capacity(N);

		unsafe {
			let src = value.as_ptr();
			let dst = this.raw.as_mut_ptr();

			copy_nonoverlapping(src, dst, N);
		}

		unsafe { this.raw.set_len(N) };

		this
	}
}

impl<K, V, A> Hash for IdentityMap<K, V, A>
where
	K: Hash,
	V: Hash,
	A: Allocator,
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_slice().hash(state);
	}
}

impl<K, V, A> Index<&K> for IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	type Output = V;

	#[inline(always)]
	#[track_caller]
	fn index(&self, index: &K) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<K, V, A> IndexMut<&K> for IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	#[inline(always)]
	#[track_caller]
	fn index_mut(&mut self, index: &K) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

impl<K, V, A: Allocator> IntoIterator for IdentityMap<K, V, A> {
	type Item = (K, V);

	type IntoIter = IntoIter<K, V, A>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		IntoIter::new(self)
	}
}

impl<'a, K, V, A: Allocator> IntoIterator for &'a IdentityMap<K, V, A> {
	type Item = &'a (K, V);

	type IntoIter = Iter<'a, K, V>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, K, V, A: Allocator> IntoIterator for &'a mut IdentityMap<K, V, A> {
	type Item = &'a mut (K, V);

	type IntoIter = IterMut<'a, K, V>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}
