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

#[cfg(test)]
mod test;

use crate::{IntoIter, Iter, IterMut};

use alloc::alloc::{Allocator, Global};
use core::alloc::Layout;
use core::any::type_name;
use core::hash::{Hash, Hasher};
use core::mem::{swap, ManuallyDrop};
use core::ops::{Index, IndexMut};
use core::ptr::{copy, drop_in_place, NonNull};
use core::slice;

/// An allocated identity map.
///
/// This map associates specific keys with specific values.
/// Unlike other maps such as `HashMap`, this type only transforms keys as if the [`identity`](core::convert::identity) function was used.
pub struct IdentityMap<K, V, A: Allocator = Global> {
	alloc: A,

	cap: isize,
	len: usize,
	ptr: NonNull<(K, V)>,
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
	pub unsafe fn from_raw_parts(ptr: *mut (K, V), cap: usize, len: usize) -> Self {
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
		Self {
			alloc,

			cap: Default::default(),
			len: Default::default(),
			ptr: NonNull::dangling(),
		}
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
		let mut this = Self::new_in(alloc);
		this.allocate(count);

		this
	}

	/// Constructs a new identity map from raw parts.
	///
	/// # Safety
	///
	/// The provided parts must have been previously been deconstructed from another identity map.
	#[inline(always)]
	#[must_use]
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

	/// Allocates the identity map.
	///
	/// Note that calling this method will leak any existing memory allocation made by this map -- if any.
	///
	/// # Panics
	///
	/// This method will panic if `count` is greater than [`isize::MAX`].
	#[inline]
	fn allocate(&mut self, count: usize) {
		assert!(count <= isize::MAX as usize);

		let layout = match Layout::array::<(K, V)>(count) {
			Ok(layout) => layout,

			Err(e) => {
				let type_name = type_name::<(K, V)>();

				panic!("unable to create layout for `[{type_name}; {count}]`: {e}");
			}
		};

		let ptr = match self.alloc.allocate(layout) {
			Ok(ptr) => ptr,

			Err(e) => panic!("unable to allocate: {e}"),
		};

		self.cap = ptr.len() as isize;
		self.len = Default::default();
		self.ptr = ptr.cast();
	}

	/// Reserves additional capacity for the map.
	///
	/// # Panics
	///
	/// This method will panic if the internal buffer could not be grown.
	/// It will also panic if the new capacity of the map is greater than [`isize::MAX`].
	#[inline]
	pub fn reserve(&mut self, count: usize) {
		if self.cap < 0x0 {
			self.allocate(count);
			return;
		}

		let old_cap = self.capacity();
		let new_cap = self.capacity() + count;

		assert!(new_cap <= isize::MAX as usize);

		let old_layout = Layout::array::<(K, V)>(old_cap).unwrap();

		let new_layout = match Layout::array::<(K, V)>(new_cap) {
			Ok(layout) => layout,

			Err(e) => {
				let type_name = type_name::<(K, V)>();

				panic!("unable to create layout for `[{type_name}; {new_cap}]`: {e}");
			}
		};

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

		self.cap = ptr.len() as isize;
		self.ptr = ptr.cast();
	}

	/// Borrows the map's allocator.
	#[inline(always)]
	#[must_use]
	pub fn allocator(&self) -> &A {
		self.alloc.by_ref()
	}

	/// Gets a iterator of the containedf key/value pairs.
	#[inline(always)]
	pub fn iter(&self) -> Iter<K, V> {
		Iter::new(self.as_slice())
	}

	/// Gets a mutable iterator of the contained key/value pairs.
	#[inline(always)]
	pub fn iter_mut(&mut self) -> IterMut<K, V> {
		IterMut::new(self.as_mut_slice())
	}

	/// Retrieves the total capacity of the map.
	///
	/// Remember that this capacity can -- if needed to -- be increased using the [`reserve`](Self::reserve) method.
	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		let mask = usize::MAX ^ 0x1;

		(self.cap as usize) & mask
	}

	/// Retrieves the current length of the map.
	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.len
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
		self.ptr.as_ptr().cast_const()
	}

	/// Gets a mutable pointer to the map buffer.
	///
	/// Note that this pointer may necessarily be dangling if the map isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		self.ptr.as_ptr()
	}

	/// Gets a slice over the map's key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[(K, V)] {
		let len = self.len();
		let ptr = self.as_ptr();

		unsafe { slice::from_raw_parts(ptr, len) }
	}

	/// Gets a mutable slice over the map's key/value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
		let len = self.len();
		let ptr = self.as_mut_ptr();

		unsafe { slice::from_raw_parts_mut(ptr, len) }
	}

	/// Deconstructs the map into its raw parts.
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

impl<K, V, A> IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	/// Inserts a new key/value pair into the map.
	///
	/// If the provided key already exists in the map, then its associated value is simply updated.
	/// The previous value is in that case returned from this method.
	///
	/// # Panics
	///
	/// If the map did not already hold `key` as a key and could not grow its buffer to accommodate the `key` & `value` pair, then this method will panic.
	#[inline]
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

		// Write new pair into the new slot.

		unsafe {
			// SAFETY: `len` will always be within bounds as we
			// have just reserved an extra byte as well.
			let ptr = self.as_mut_ptr().add(self.len());

			ptr.write((key, value));
		}

		// Increment the length counter (by one).

		self.len += 0x1;

		// Return nothing as the key wasn't already pre-
		// sent.

		None
	}

	/// Removes the whole pair associated with the specific key.
	///
	/// The associated value is returned from this method.
	/// If no pair existed with the provided key, then this method will instead return a [`None`] instance.
	#[inline]
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

		// Retrieve the value from the buffer.

		let (_, value) = unsafe {
			let ptr = self.as_mut_ptr().add(index);

			ptr.read()
		};

		// Shift every following item that is still alive
		// up front (by one).

		unsafe {
			let len = self.len() - index;
			let dst = self.as_mut_ptr().add(index);
			let src = self.as_ptr().add(index).add(0x1);

			copy(src, dst, len);
		}

		// Decrease the length counter (by one).

		self.len -= 0x1;

		// Return the value.

		Some(value)
	}

	/// Borrows the associated value of a key.
	#[inline(always)]
	#[must_use]
	pub fn get(&self, key: &K) -> Option<&V> {
		for (other_key, other_value) in self {
			if *other_key == *key { return Some(other_value) };
		}

		None
	}

	/// Mutably borrows the associated value of a key.
	#[inline(always)]
	#[must_use]
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
		matches!(self.get(key), Some(..))
	}
}

impl<K, V, A: Allocator + Default> Default for IdentityMap<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		Self::new_in(Default::default())
	}
}

impl<K, V, A: Allocator> Drop for IdentityMap<K, V, A> {
	#[inline]
	fn drop(&mut self) {
		if self.cap < 0x0 { return };

		let remaining: *mut [(K, V)] = unsafe {
			let len = self.len();
			let ptr = self.as_mut_ptr();

			slice::from_raw_parts_mut(ptr, len)
		};

		unsafe { drop_in_place(remaining) };

		unsafe {
			let layout = Layout::array::<(K, V)>(self.cap as usize).unwrap();

			let ptr = self.ptr.cast();

			self.alloc.deallocate(ptr, layout);
		}
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
		let data = unsafe {
			let len = self.len();
			let ptr = self.ptr.as_ptr().cast_const();

			slice::from_raw_parts(ptr, len)
		};

		data.hash(state)
	}
}

impl<K, V, A> Index<&K> for IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	type Output = V;

	#[inline(always)]
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
	fn index_mut(&mut self, index: &K) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

impl<K, V, A> IntoIterator for IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	type Item = (K, V);

	type IntoIter = IntoIter<K, V, A>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		let (ptr, cap, len, alloc) = self.into_raw_parts_with_allow();

		unsafe { IntoIter::new(
			ptr,
			cap,
			len,
			alloc,
		) }
	}
}

impl<'a, K, V, A> IntoIterator for &'a IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	type Item = &'a (K, V);

	type IntoIter = Iter<'a, K, V>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, K, V, A> IntoIterator for &'a mut IdentityMap<K, V, A>
where
	K: Eq,
	A: Allocator,
{
	type Item = &'a mut (K, V);

	type IntoIter = IterMut<'a, K, V>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

unsafe impl<K, V, A> Send for IdentityMap<K, V, A>
where
	K: Send,
	V: Send,
	A: Allocator  + Send,
{ }

unsafe impl<K, V, A> Sync for IdentityMap<K, V, A>
where
	K: Sync,
	V: Sync,
	A: Allocator  + Sync,
{ }
