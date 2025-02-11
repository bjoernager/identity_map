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

use crate::IdentityMap;

use core::sync::atomic::{AtomicU8, Ordering};

#[allow(clippy::len_zero)]
#[test]
fn test_identity_map() {
	assert_eq!(size_of::<IdentityMap<usize, u8>>(), size_of::<Option<IdentityMap<(),()>>>());

	let mut map = IdentityMap::<u32, u32>::new();

	assert!(map.capacity() == 0x0);
	assert!(map.len() == 0x0);
	assert!(map.is_empty());

	map.reserve(0x100);

	assert!(map.capacity() >= 0x100);
	assert!(map.len() == 0x0);
	assert!(map.is_empty());
	assert!(!map.contains_key(&u32::MIN));
	assert!(!map.contains_key(&u32::MAX));
	assert!(!map.contains_key(&(i32::MAX as u32)));

	assert_eq!(map.insert(u32::MIN, u32::MAX), None);

	assert!(map.capacity() >= 0x100);
	assert!(map.len() == 0x1);
	assert!(!map.is_empty());
	assert!(map.contains_key(&u32::MIN));
	assert!(!map.contains_key(&u32::MAX));
	assert!(!map.contains_key(&(i32::MAX as u32)));

	assert_eq!(map.insert(u32::MAX, u32::MIN), None);

	assert!(map.capacity() >= 0x100);
	assert!(map.len() == 0x2);
	assert!(!map.is_empty());
	assert!(map.contains_key(&u32::MIN));
	assert!(map.contains_key(&u32::MAX));
	assert!(!map.contains_key(&(i32::MAX as u32)));

	assert_eq!(map.remove(&u32::MIN), Some(u32::MAX));

	assert!(map.capacity() >= 0x100);
	assert!(map.len() == 0x1);
	assert!(!map.is_empty());
	assert!(!map.contains_key(&u32::MIN));
	assert!(map.contains_key(&u32::MAX));
	assert!(!map.contains_key(&(i32::MAX as u32)));

	assert_eq!(map.remove(&u32::MAX), Some(u32::MIN));

	assert!(map.capacity() >= 0x100);
	assert!(map.len() == 0x0);
	assert!(map.is_empty());
	assert!(!map.contains_key(&u32::MIN));
	assert!(!map.contains_key(&u32::MAX));
	assert!(!map.contains_key(&(i32::MAX as u32)));

	assert_eq!(map.remove(&u32::MIN), None);

	assert!(map.capacity() >= 0x100);
	assert!(map.len() == 0x0);
	assert!(map.is_empty());
	assert!(!map.contains_key(&u32::MIN));
	assert!(!map.contains_key(&u32::MAX));
	assert!(!map.contains_key(&(i32::MAX as u32)));
}

#[test]
fn test_identity_map_clone() {
	let mut m0 = IdentityMap::<usize, char>::new();

	assert_eq!(m0.insert(0x0, '\u{20A3}'), None);
	assert_eq!(m0.insert(0x1, '\u{2611}'), None);

	let mut m1 = m0.clone();

	assert_eq!(m0.remove(&0x0), Some('\u{20A3}'));
	assert_eq!(m1.remove(&0x0), Some('\u{20A3}'));
	assert_eq!(m0.remove(&0x1), Some('\u{2611}'));
	assert_eq!(m1.remove(&0x1), Some('\u{2611}'));
}

#[test]
fn test_identity_map_drop() {
	static COUNTER: AtomicU8 = AtomicU8::new(0x0);

	#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
	struct Foo;

	impl Drop for Foo {
		fn drop(&mut self) {
			COUNTER.fetch_add(0x1, Ordering::Release);
		}
	}

	let mut map = IdentityMap::new();

	map.insert(Foo, Foo);
	map.insert(Foo, Foo);
	map.insert(Foo, Foo);
	map.insert(Foo, Foo);

	drop(map);

	assert_eq!(COUNTER.load(Ordering::Acquire), 0x8);
}

#[test]
fn test_identity_map_from_array() {
	let data = [(true, false), (false, true)];

	let map = IdentityMap::from(data);

	assert_eq!(map.get(&true),  Some(&false));
	assert_eq!(map.get(&false), Some(&true));
}

#[test]
fn test_identity_map_iter() {
	let mut map = IdentityMap::<u8, u8>::new();

	assert_eq!(map.insert(0xFF, 0x00), None);
	assert_eq!(map.insert(0x7F, 0x80), None);
	assert_eq!(map.insert(0x00, 0xFF), None);

	let mut iter = map.iter();

	assert_eq!(iter.next(), Some(&(0x00, 0xFF)));
	assert_eq!(iter.next(), Some(&(0x7F, 0x80)));
	assert_eq!(iter.next(), Some(&(0xFF, 0x00)));
	assert_eq!(iter.next(), None);

	let mut iter = map.iter_mut();

	assert_eq!(iter.next(), Some(&mut (0x00, 0xFF)));
	assert_eq!(iter.next(), Some(&mut (0x7F, 0x80)));
	assert_eq!(iter.next(), Some(&mut (0xFF, 0x00)));
	assert_eq!(iter.next(), None);

	let mut iter = map.into_iter();

	assert_eq!(iter.next(), Some((0x00, 0xFF)));
	assert_eq!(iter.next(), Some((0x7F, 0x80)));
	assert_eq!(iter.next(), Some((0xFF, 0x00)));
	assert_eq!(iter.next(), None);
}
