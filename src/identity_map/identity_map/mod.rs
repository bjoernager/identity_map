// Copyright 2025 Gabriel Bjørnager Jensen.

#[cfg(test)]
mod test;

#[cfg(feature = "serde")]
mod serde;

use crate::identity_map::{
	Drain,
	IntoIter,
	IntoKeys,
	IntoValues,
	Iter,
	IterMut,
	Keys,
	Values,
	ValuesMut,
};

use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::vec::Vec;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::mem::swap;
use core::ops::Index;

/// An ordered identity map.
///
/// This map associates specific keys with specific values, whereby each key is unique.
///
/// Unlike other maps such as [`HashMap`](std::collections::HashMap), this type only transforms keys as if the [`identity`](core::convert::identity) function was used.
///
/// As this map is ordered, lookup, insertion, and removal performance are all *O*(log<sub>2</sub> *n*) .
#[derive(Clone)]
pub struct IdentityMap<K, V, A: Allocator = Global> {
	buf: Vec<(K, V), A>,
}

impl<K, V> IdentityMap<K, V> {
	/// Constructs a new, empty identity map.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const fn new() -> Self {
		Self::new_in(Global)
	}

	/// Preallocates a new identity map.
	///
	/// # Panics
	///
	/// If `[(K, V); cap]` could not be allocated using the global allocator, then this function will panic.
	///
	/// This function will also panic if `cap` is greater than [`isize::MAX`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity(cap: usize) -> Self {
		Self::with_capacity_in(cap, Global)
	}

	/// Constructs a new identity map from raw parts.
	///
	/// # Safety
	///
	/// See [`Vec::from_raw_parts`](alloc::vec::Vec::from_raw_parts).
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts(ptr: *mut (K, V), cap: usize, len: usize) -> Self {
		// SAFETY: Caller guarantees the validity of the
		// parts.
		unsafe { Self::from_raw_parts_in(ptr, cap, len, Global) }
	}
}

impl<K, V, A: Allocator> IdentityMap<K, V, A> {
	/// Constructs a new, empty identity map with a specific allocator.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub const fn new_in(alloc: A) -> Self {
		let buf = Vec::new_in(alloc);
		Self { buf }
	}

	/// Preallocates a new identity map with a specific allocator.
	///
	/// # Panics
	///
	/// If `[(K, V); cap]` could not be allocated with the given allocator, then this function will panic.
	///
	/// This function will also panic if `cap` is greater than [`isize::MAX`].
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn with_capacity_in(cap: usize, alloc: A) -> Self {
		let buf = Vec::with_capacity_in(cap, alloc);
		Self { buf }
	}

	/// Constructs a new identity map from raw parts.
	///
	/// # Safety
	///
	/// See [`Vec::from_raw_parts_in`](alloc::vec::Vec::from_raw_parts_in).
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub unsafe fn from_raw_parts_in(
		ptr:   *mut (K, V),
		cap:   usize,
		len:   usize,
		alloc: A,
	) -> Self {
		let buf = unsafe { Vec::from_raw_parts_in(ptr, cap, len, alloc) };
		Self { buf }
	}

	/// Retains only key-value pairs as specified by a predicate.
	///
	/// In other words, each pair `(k, v)` where `!f(k, v)` is true.
	///
	/// # Panics
	///
	/// Panics if `f` panics.
	#[inline(always)]
	#[track_caller]
	pub fn retain<F: FnMut(&K, &mut V) -> bool>(&mut self, mut f: F) {
		self.buf.retain_mut(|(k, v)| f(&*k, v));
	}

	/// Clears the map.
	///
	/// All contained keys and values are dropped after a call to this method.
	/// The length counter is then reset to zero.
	#[inline(always)]
	pub fn clear(&mut self) {
		self.buf.clear();
	}

	/// Reserves additional capacity for the map.
	///
	/// # Panics
	///
	/// This method will panic if the internal buffer could not be grown.
	/// It will also panic if the new capacity of the map is greater than [`isize::MAX`].
	#[inline(always)]
	#[track_caller]
	pub fn reserve(&mut self, len: usize) {
		self.buf.reserve(len);
	}

	/// Shrinks the map to a specified length.
	///
	/// The capacity is shrunk such that it exactly contains the current data.
	///
	/// # Panics
	///
	/// If the provided capacity is greater than the current, then this method will panic.
	#[inline(always)]
	#[track_caller]
	pub fn shrink_to(&mut self, cap: usize) {
		self.buf.shrink_to(cap)
	}

	/// Shrinks the map to the current length.
	///
	/// The capacity is shrunk such that it exactly contains the current data.
	#[inline(always)]
	pub fn shrink_to_fit(&mut self) {
		self.buf.shrink_to_fit()
	}

	/// Borrows the map's allocator.
	#[inline(always)]
	#[must_use]
	pub fn allocator(&self) -> &A {
		self.buf.allocator()
	}

	/// Drains the key-value pairs of the map.
	#[inline(always)]
	pub fn drain(&mut self) -> Drain<K, V, A> {
		Drain::new(self)
	}

	/// Gets an iterator of the contained key-value pairs.
	#[inline(always)]
	pub fn iter(&self) -> Iter<K, V> {
		Iter::new(self)
	}

	/// Gets a mutable iterator of the contained key-value pairs.
	#[inline(always)]
	pub fn iter_mut(&mut self) -> IterMut<K, V> {
		IterMut::new(self)
	}

	/// Gets an iterator of the contained keys.
	#[inline(always)]
	pub fn keys(&self) -> Keys<K, V> {
		Keys::new(self)
	}

	/// Gets an iterator of the contained values.
	#[inline(always)]
	pub fn values(&self) -> Values<K, V> {
		Values::new(self)
	}

	/// Gets a mutable iterator of the contained values.
	#[inline(always)]
	pub fn values_mut(&mut self) -> ValuesMut<K, V> {
		ValuesMut::new(self)
	}

	/// Retrieves the total capacity of the map.
	///
	/// Remember that this capacity can -- if needed to -- be increased using the [`reserve`](Self::reserve) method.
	#[inline(always)]
	#[must_use]
	pub fn capacity(&self) -> usize {
		self.buf.capacity()
	}

	/// Retrieves the current length of the map.
	#[inline(always)]
	#[must_use]
	pub fn len(&self) -> usize {
		self.buf.len()
	}

	/// Tests if the map is empty.
	#[inline(always)]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.buf.is_empty()
	}

	/// Gets a pointer to the map buffer.
	///
	/// Note that this pointer may necessarily be dangling if the map isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_ptr(&self) -> *const (K, V) {
		self.buf.as_ptr()
	}

	/// Gets a mutable pointer to the map buffer.
	///
	/// Note that this pointer may necessarily be dangling if the map isn't currently in an allocated state.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut (K, V) {
		self.buf.as_mut_ptr()
	}

	/// Gets a slice over the map's key-value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[(K, V)] {
		self.buf.as_slice()
	}

	/// Gets a mutable slice over the map's key-value pairs.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [(K, V)] {
		self.buf.as_mut_slice()
	}

	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_vec(&self) -> &Vec<(K, V), A> {
		&self.buf
	}

	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_mut_vec(&mut self) -> &mut Vec<(K, V), A> {
		&mut self.buf
	}

	/// Gets an iterator of the contained keys.
	#[inline(always)]
	pub fn into_keys(self) -> IntoKeys<K, V, A> {
		IntoKeys::new(self)
	}

	/// Gets an iterator of the contained values.
	#[inline(always)]
	pub fn into_values(self) -> IntoValues<K, V, A> {
		IntoValues::new(self)
	}

	#[inline(always)]
	#[must_use]
	pub(crate) fn into_vec(self) -> Vec<(K, V), A> {
		self.buf
	}
}

impl<K, V, A> IdentityMap<K, V, A>
where
	K: Ord,
	A: Allocator,
{
	/// Inserts all key-value pairs from another map, overwriting duplicates.
	///
	/// The other map `other` will be completely cleared
	#[inline]
	#[track_caller]
	pub fn append(&mut self, other: &mut Self)
	where
		A: Clone,
	{
		if other.is_empty() { return };

		if self.is_empty() {
			swap(self, other);
			return;
		}

		self.extend(other.drain());
	}

	/// Replaces a key-value pair.
	#[inline]
	pub(crate) fn replace(&mut self, mut key: K, mut value: V) -> Option<(K, V)> {
		let slot = {
			let index = self.get_index(&key).ok()?;
			&mut self.buf[index]
		};

		swap(&mut slot.0, &mut key);
		swap(&mut slot.1, &mut value);

		Some((key, value))
	}

	/// Inserts a new key-value pair into the map.
	///
	/// If the provided key already exists in the map, then its associated value is simply updated.
	/// The previous value is in that case returned from this method.
	///
	/// # Panics
	///
	/// If the map did not already hold `key` as a key and could not grow its buffer to accommodate the `key` & `value` pair, then this method will panic.
	/// This includes whether growing the buffer would make its capacity exceed `isize::MAX`, or whether increasing the length would.
	#[inline]
	pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
		// Check if we already have the key, and if so up-
		// date its value and short-circuit.

		let index = match self.get_index(&key) {
			Ok(index) => {
				let (_, other_value) = self.buf.get_mut(index).unwrap();

				swap(other_value, &mut value);
				return Some(value);
			}

			Err(index) => index,
		};

		// Insert the new pair into the slot.

		self.buf.insert(index, (key, value));

		// Return nothing as the key wasn't already pre-
		// sent.

		None
	}

	/// Removes the whole pair associated with the specific key.
	///
	/// The associated value is returned from this method.
	/// If no pair existed with the provided key, then this method will instead return a [`None`] instance.
	#[inline]
	pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		// Search for the given key. Short-circuit if it
		// wasn't present.

		let index = match self.get_index(key) {
			Ok(index) => index,

			_ => return None,
		};

		// Retrieve the pair from the buffer.

		let (key, value) = self.buf.remove(index);
		Some((key, value))
	}

	/// Removes the whole pair associated with the specific key.
	///
	/// The associated value is returned from this method.
	/// If no pair existed with the provided key, then this method will instead return a [`None`] instance.
	#[inline(always)]
	#[must_use]
	pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		self.remove_entry(key).map(|(_, v)| v)
	}

	/// Pops the first key-value pair.
	#[inline(always)]
	#[must_use]
	pub fn pop_first(&mut self) -> Option<(K, V)> {
		if self.is_empty() { return None };

		let entry = self.buf.remove(0x0);
		Some(entry)
	}

	/// Pops the last key-value pair.
	#[inline(always)]
	#[must_use]
	pub fn pop_last(&mut self) -> Option<(K, V)> {
		self.buf.pop()
	}

	/// Gets the raw index of the specified key.
	///
	/// If the key was found in the internal buffer, then an instance of [`Ok`] is returned.
	/// Otherwise, an index appropriate for inserting said key is wrapped as an [`Err`] instance.
	#[inline(always)]
	fn get_index<Q>(&self, key: &Q) -> Result<usize, usize>
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		self.buf.binary_search_by(|(other_key, _)| {
			let other_key = Borrow::<Q>::borrow(other_key);
			other_key.cmp(key)
		})
	}

	/// Borrows a key-value pair.
	#[inline(always)]
	#[must_use]
	pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		match self.get_index(key) {
			Ok(index) => {
				let (key, value) = self.buf.get(index).unwrap();
				Some((key, value))
			}

			_ => None,
		}
	}

	/// Borrows the associated value of a key.
	#[inline(always)]
	#[must_use]
	pub fn get<Q>(&self, key: &Q) -> Option<&V>
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		match self.get_index(key) {
			Ok(index) => {
				let (_, value) = self.buf.get(index).unwrap();
				Some(value)
			}

			_ => None,
		}
	}

	/// Mutably borrows the associated value of a key.
	#[inline(always)]
	#[must_use]
	pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		match self.get_index(key) {
			Ok(index) => {
				let (_, value) = self.buf.get_mut(index).unwrap();
				Some(value)
			}

			_ => None,
		}
	}

	/// Borrows the first key-value pair.
	#[inline(always)]
	#[must_use]
	pub fn first_key_value(&self) -> Option<(&K, &V)> {
		self.buf.first().map(|(k, v)| (k, v))
	}

	/// Borrows the last key-value pair.
	#[inline(always)]
	#[must_use]
	pub fn last_key_value(&self) -> Option<(&K, &V)> {
		self.buf.last().map(|(k, v)| (k, v))
	}

	/// Checks if the map contains the specified key.
	#[inline(always)]
	#[must_use]
	pub fn contains_key<Q>(&self, key: &Q) -> bool
	where
		K: Borrow<Q>,
		Q: Ord + ?Sized,
	{
		self.get_index(key).is_ok()
	}
}

impl<K, V, A> Debug for IdentityMap<K, V, A>
where
	K: Debug,
	V: Debug,
	A: Allocator,
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Debug::fmt(self.as_slice(), f)
	}
}

impl<K, V, A: Allocator + Default> Default for IdentityMap<K, V, A> {
	#[inline(always)]
	fn default() -> Self {
		Self::new_in(Default::default())
	}
}

impl<K, V, A> Eq for IdentityMap<K, V, A>
where
	K: Eq,
	V: Eq,
	A: Allocator,
{ }

impl<K, V, A> Extend<(K, V)> for IdentityMap<K, V, A>
where
	K: Ord,
	A: Allocator,
{
	#[inline]
	fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
		let iter = iter.into_iter();

		self.reserve(iter.size_hint().0);

		for (key, value) in iter {
			self.insert(key, value);
		}
	}
}

impl<K, V, A, const N: usize> From<[(K, V); N]> for IdentityMap<K, V, A>
where
	K: Ord,
	A: Allocator + Default,
{
	#[inline(always)]
	fn from(value: [(K, V); N]) -> Self {
		value.into_iter().collect()
	}
}

impl<K, V, A> FromIterator<(K, V)> for IdentityMap<K, V, A>
where
	K: Ord,
	A: Allocator + Default,
{
	#[inline]
	fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
		let iter = iter.into_iter();

		let mut this = Self::with_capacity_in(iter.size_hint().0, Default::default());

		for (key, value) in iter {
			this.insert(key, value);
		}

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

impl<K, V, A, Q> Index<&Q> for IdentityMap<K, V, A>
where
	K: Borrow<Q> + Ord,
	A: Allocator,
	Q: Ord + ?Sized,
{
	type Output = V;

	#[inline(always)]
	#[track_caller]
	fn index(&self, index: &Q) -> &Self::Output {
		self.get(index).unwrap()
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
	type Item = (&'a K, &'a V);

	type IntoIter = Iter<'a, K, V>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a, K, V, A: Allocator> IntoIterator for &'a mut IdentityMap<K, V, A> {
	type Item = (&'a K, &'a mut V);

	type IntoIter = IterMut<'a, K, V>;

	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		self.iter_mut()
	}
}

impl<K, V, A> Ord for IdentityMap<K, V, A>
where
	K: Ord,
	V: Ord,
	A: Allocator,
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering {
		self.buf.cmp(&other.buf)
	}
}

impl<K, V, A> PartialEq for IdentityMap<K, V, A>
where
	K: PartialEq,
	V: PartialEq,
	A: Allocator,
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.buf == other.buf
	}
}

impl<K, V, A> PartialOrd for IdentityMap<K, V, A>
where
	K: PartialOrd,
	V: PartialOrd,
	A: Allocator,
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.buf.partial_cmp(&other.buf)
	}
}
