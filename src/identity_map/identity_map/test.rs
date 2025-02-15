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

use alloc::vec::Vec;
use bincode::{deserialize_from, serialize_into};
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
fn test_identity_map_drain() {
	let mut map = IdentityMap::<&'static str, usize>::from([
		("!",     0x00),
		("())",   0x00),
		("bool",  0x01),
		("char",  0x04),
		("u8",    0x01),
		("u16",   0x02),
		("u32",   0x04),
		("u64",   0x08),
		("u128",  0x10),
		("usize", 0x02),
		("i8",    0x01),
		("i16",   0x02),
		("i32",   0x04),
		("i64",   0x08),
		("i128",  0x10),
		("isize", 0x02),
		("f16",   0x02),
		("f32",   0x04),
		("f64",   0x08),
		("f128",  0x10),
	]);

	assert_eq!(map.len(), 0x14);

	let mut iter = map.drain();

	assert_eq!(iter.next(), Some(("!",     0x00)));
	assert_eq!(iter.next(), Some(("())",   0x00)));
	assert_eq!(iter.next(), Some(("bool",  0x01)));
	assert_eq!(iter.next(), Some(("char",  0x04)));
	assert_eq!(iter.next(), Some(("f128",  0x10)));
	assert_eq!(iter.next(), Some(("f16",   0x02)));
	assert_eq!(iter.next(), Some(("f32",   0x04)));
	assert_eq!(iter.next(), Some(("f64",   0x08)));
	assert_eq!(iter.next(), Some(("i128",  0x10)));
	assert_eq!(iter.next(), Some(("i16",   0x02)));
	assert_eq!(iter.next(), Some(("i32",   0x04)));
	assert_eq!(iter.next(), Some(("i64",   0x08)));
	assert_eq!(iter.next(), Some(("i8",    0x01)));
	assert_eq!(iter.next(), Some(("isize", 0x02)));
	assert_eq!(iter.next(), Some(("u128",  0x10)));
	assert_eq!(iter.next(), Some(("u16",   0x02)));
	assert_eq!(iter.next(), Some(("u32",   0x04)));
	assert_eq!(iter.next(), Some(("u64",   0x08)));
	assert_eq!(iter.next(), Some(("u8",    0x01)));
	assert_eq!(iter.next(), Some(("usize", 0x02)));
	assert_eq!(iter.next(), None);

	drop(iter);

	assert!(map.is_empty());
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
	let data0 = [(true, true), (true, false), (false, true), (false, false), (true, false), (false, true)];
	let data1 = [(false, true), (true, false)];

	let map0: IdentityMap<_, _> = IdentityMap::from(data0);
	let map1: IdentityMap<_, _> = IdentityMap::from(data1);

	assert_eq!(map0, map1);

	assert_eq!(map0.get(&true),  Some(&false));
	assert_eq!(map0.get(&false), Some(&true));

	assert_eq!(map1.get(&true),  Some(&false));
	assert_eq!(map1.get(&false), Some(&true));
}

#[test]
fn test_identity_map_iter() {
	let mut map = IdentityMap::<u8, u8>::from([
		(0xFF, 0x00),
		(0x7F, 0x80),
		(0x00, 0xFF),
	]);

	let mut iter = map.iter();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some((&0x00, &0xFF)));
	assert_eq!(iter.next(), Some((&0x7F, &0x80)));
	assert_eq!(iter.next(), Some((&0xFF, &0x00)));
	assert_eq!(iter.next(), None);

	let mut iter = map.iter_mut();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some((&0x00, &mut 0xFF)));
	assert_eq!(iter.next(), Some((&0x7F, &mut 0x80)));
	assert_eq!(iter.next(), Some((&0xFF, &mut 0x00)));
	assert_eq!(iter.next(), None);

	let mut iter = map.keys();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some(&0x00));
	assert_eq!(iter.next(), Some(&0x7F));
	assert_eq!(iter.next(), Some(&0xFF));
	assert_eq!(iter.next(), None);

	let mut iter = map.values();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some(&0xFF));
	assert_eq!(iter.next(), Some(&0x80));
	assert_eq!(iter.next(), Some(&0x00));
	assert_eq!(iter.next(), None);

	let mut iter = map.values_mut();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some(&mut 0xFF));
	assert_eq!(iter.next(), Some(&mut 0x80));
	assert_eq!(iter.next(), Some(&mut 0x00));
	assert_eq!(iter.next(), None);

	let mut iter = map.clone().into_iter();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some((0x00, 0xFF)));
	assert_eq!(iter.next(), Some((0x7F, 0x80)));
	assert_eq!(iter.next(), Some((0xFF, 0x00)));
	assert_eq!(iter.next(), None);

	let mut iter = map.clone().into_keys();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some(0x00));
	assert_eq!(iter.next(), Some(0x7F));
	assert_eq!(iter.next(), Some(0xFF));
	assert_eq!(iter.next(), None);

	let mut iter = map.clone().into_values();

	assert_eq!(iter.len(), 0x3);

	assert_eq!(iter.next(), Some(0xFF));
	assert_eq!(iter.next(), Some(0x80));
	assert_eq!(iter.next(), Some(0x00));
	assert_eq!(iter.next(), None);
}

#[test]
fn test_identity_set_serialise_deserialise() {
	let input = IdentityMap::<char, [u8; 0x2]>::from([
		('Æ', *b"AE"),
		('Ø', *b"OE"),
		('Å', *b"AA"),
		('æ', *b"ae"),
		('ø', *b"oe"),
		('å', *b"aa"),
	]);

	let mut buf = Vec::new();

	serialize_into(&mut buf, &input).unwrap();

	let output: IdentityMap<char, [u8; 0x2]> = deserialize_from(&*buf).unwrap();

	assert_eq!(output, input);
}
