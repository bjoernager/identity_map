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
use crate::identity_set::{
	Difference,
	Intersection,
	IntoIter,
	Iter,
	SymmetricDifference,
	Union,
};

use allocator_api2::alloc::{Allocator, Global};
use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{BitAnd, BitOr, BitXor, Sub};

/// An ordered identity set.
///
/// This set records a list of keys wherein each key is unique.
///
/// Unlike other sets such as [`HashSet`](std::collections::HashSet), this type only transforms keys as if the [`identity`](core::convert::identity) function was used.
#[repr(transparent)]
#[derive(Clone)]
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

	/// Retains only keys as specified by a predicate.
	///
	/// In other words, each key `k` where `!f(k)` is true.
	///
	/// # Panics
	///
	/// Panics if `f` panics.
	#[inline(always)]
	#[track_caller]
	pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
		self.map.retain(|k, _| f(k));
	}

	/// Clears the set.
	///
	/// All contained keys are dropped after a call to this method.
	/// The length counter is then reset to zero.
	#[inline(always)]
	pub fn clear(&mut self) {
		self.map.clear();
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

	/// Shrinks the set to a specified length.
	///
	/// The capacity is shrunk such that it exactly contains the current data.
	///
	/// # Panics
	///
	/// If the provided capacity is greater than the current, then this method will panic.
	#[inline(always)]
	#[track_caller]
	pub fn shrink_to(&mut self, cap: usize) {
		self.map.shrink_to(cap)
	}

	/// Shrinks the set to the current length.
	///
	/// The capacity is shrunk such that it exactly contains the current data.
	#[inline(always)]
	pub fn shrink_to_fit(&mut self) {
		self.map.shrink_to_fit()
	}

	/// Borrows the set's allocator.
	#[inline(always)]
	#[must_use]
	pub fn allocator(&self) -> &A {
		self.map.allocator()
	}

	/// Gets an iterator of the contained keys.
	#[inline(always)]
	pub fn iter(&self) -> Iter<T> {
		Iter::new(self)
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

	/// Gets a slice over the set's keys.
	#[inline(always)]
	#[must_use]
	pub fn as_slice(&self) -> &[T] {
		// SAFETY: `(T, ())` is transparent to `T`.
		unsafe { &*(&raw const *self.map.as_slice() as *const [T]) }
	}

	/// Gets a mutable slice over the set's keys.
	#[inline(always)]
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [T] {
		// SAFETY: `(T, ())` is transparent to `T`.
		unsafe { &mut *(&raw mut *self.map.as_mut_slice() as *mut [T]) }
	}

	/// Borrows the set as a map.
	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_map(&self) -> &IdentityMap<T, (), A> {
		&self.map
	}

	/// Mutably borrows the set as a map.
	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub(crate) fn as_mut_map(&mut self) -> &mut IdentityMap<T, (), A> {
		&mut self.map
	}

	/// Converts the set into a map.
	#[allow(unused)]
	#[inline(always)]
	#[must_use]
	pub(crate) fn into_map(self) -> IdentityMap<T, (), A> {
		self.map
	}
}

impl<T, A> IdentitySet<T, A>
where
	T: Ord,
	A: Allocator,
{
	/// Inserts a new key pair into the set.
	///
	/// If the provided key already exists in the set, then this method will return `true`.
	/// In all other cases, it will return `false`.
	///
	/// # Panics
	///
	/// If the set did not already hold `key` as a key and could not grow its buffer to accommodate the key, then this method will panic.
	#[inline(always)]
	#[track_caller]
	pub fn insert(&mut self, key: T) -> bool {
		self.map.insert(key, ()).is_some()
	}

	/// Takes a specific key out from the set.
	///
	/// If the provided key was not present in the set, then this method will instead return a [`None`] instance.
	#[inline(always)]
	#[track_caller]
	pub fn take<U>(&mut self, key: &U) -> Option<T>
	where
		T: Borrow<U>,
		U: Ord + ?Sized,
	{
		self.map.remove_entry(key).map(|(k, _)| k)
	}

	/// Remove a specific key from the set.
	///
	/// This method will return `true` if the provided key was present in the set.
	#[inline(always)]
	#[track_caller]
	pub fn remove(&mut self, key: &T) -> bool {
		self.take(key).is_some()
	}

	/// Borrows a key.
	#[inline(always)]
	#[must_use]
	#[track_caller]
	pub fn get<U>(&self, key: &U) -> Option<&T>
	where
		T: Borrow<U>,
		U: Ord + ?Sized,
	{
		self.map.get_key_value(key).map(|(k, _)| k)
	}

	/// Checks if the set contains the specified key.
	#[inline(always)]
	#[must_use]
	pub fn contains<U>(&self, key: &U) -> bool
	where
		T: Borrow<U>,
		U: Ord + ?Sized,
	{
		self.map.contains_key(key)
	}

	/// Gets an iterator denoting the [intersection](https://en.wikipedia.org/wiki/Intersection/) between two sets.
	#[inline(always)]
	pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T, A> {
		Intersection::new(self, other)
	}

	/// Gets an iterator denoting the [difference](https://en.wikipedia.org/wiki/Complement_(set_theory)#Relative_complement) between two sets.
	#[inline(always)]
	pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T, A> {
		Difference::new(self, other)
	}

	/// Gets an iterator denoting the [symmetric difference](https://en.wikipedia.org/wiki/Symmetric_difference/) between two sets
	#[inline(always)]
	pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, T, A> {
		SymmetricDifference::new(self, other)
	}

	/// Gets an iterator denoting the [union](https://en.wikipedia.org/wiki/Union_(set_theory)/) between two sets
	#[inline(always)]
	pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T, A> {
		Union::new(self, other)
	}
}

impl<T, A> BitAnd for &IdentitySet<T, A>
where
	T: Clone + Ord,
	A: Allocator + Default,
{
	type Output = IdentitySet<T, A>;

	#[inline(always)]
	fn bitand(self, rhs: Self) -> Self::Output {
		self.intersection(rhs).cloned().collect()
	}
}

impl<T, A> BitOr for &IdentitySet<T, A>
where
	T: Clone + Ord,
	A: Allocator + Default,
{
	type Output = IdentitySet<T, A>;

	#[inline(always)]
	fn bitor(self, rhs: Self) -> Self::Output {
		self.union(rhs).cloned().collect()
	}
}

impl<T, A> BitXor for &IdentitySet<T, A>
where
	T: Clone + Ord,
	A: Allocator + Default,
{
	type Output = IdentitySet<T, A>;

	#[inline(always)]
	fn bitxor(self, rhs: Self) -> Self::Output {
		self.symmetric_difference(rhs).cloned().collect()
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

impl<T, A> Eq for IdentitySet<T, A>
where
	T: Eq,
	A: Allocator,
{ }

impl<T, A: Allocator + Default> Default for IdentitySet<T, A> {
	#[inline(always)]
	fn default() -> Self {
		Self::new_in(Default::default())
	}
}

impl<T, A> Extend<T> for IdentitySet<T, A>
where
	T: Ord,
	A: Allocator,
{
	#[inline]
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		let iter = iter.into_iter();

		self.reserve(iter.size_hint().0);

		for key in iter {
			self.insert(key);
		}
	}
}

impl<T, A, const N: usize> From<[T; N]> for IdentitySet<T, A>
where
	T: Ord,
	A: Allocator + Default,
{
	#[inline(always)]
	fn from(value: [T; N]) -> Self {
		value.into_iter().collect()
	}
}

impl<T, A> FromIterator<T> for IdentitySet<T, A>
where
	T: Ord,
	A: Allocator + Default,
{
	#[inline]
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let iter = iter.into_iter();

		let mut this = Self::with_capacity_in(iter.size_hint().0, Default::default());

		for key in iter {
			this.insert(key);
		}

		this
	}
}

impl<T, A> Hash for IdentitySet<T, A>
where
	T: Hash,
	A: Allocator,
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_slice().hash(state);
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

impl<T, A> PartialEq for IdentitySet<T, A>
where
	T: PartialEq,
	A: Allocator,
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool {
		self.map == other.map
	}
}

#[cfg(feature = "serde")]
impl<T, A> serde::Serialize for IdentitySet<T, A>
where
	T: serde::Serialize,
	A: Allocator,
{
	#[inline(always)]
	fn serialize<S: serde::Serializer>(&self, serialiser: S) -> Result<S::Ok, S::Error> {
		serialiser.collect_seq(self.iter())
	}
}

impl<T, A> Sub for &IdentitySet<T, A>
where
	T: Clone + Ord,
	A: Allocator + Default,
{
	type Output = IdentitySet<T, A>;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		self.difference(rhs).cloned().collect()
	}
}
