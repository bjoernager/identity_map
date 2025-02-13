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

use core::num::NonZero;

use crate::identity_set::IdentitySet;

#[allow(clippy::len_zero)]
#[test]
fn test_identity_set() {
	assert_eq!(size_of::<IdentitySet<usize>>(), size_of::<Option<IdentitySet<()>>>());

	let mut set = IdentitySet::<char>::new();

	assert!(set.capacity() == 0x0);
	assert!(set.len() == 0x0);
	assert!(set.is_empty());

	set.reserve(0x100);

	assert!(set.capacity() >= 0x100);
	assert!(set.len() == 0x0);
	assert!(set.is_empty());
	assert!(!set.contains(&'\u{1F12F}'));
	assert!(!set.contains(&'\u{03FD}'));
	assert!(!set.contains(&'\0'));

	assert!(!set.insert('\u{1F12F}'));

	assert!(set.capacity() >= 0x100);
	assert!(set.len() == 0x1);
	assert!(!set.is_empty());
	assert!(set.contains(&'\u{1F12F}'));
	assert!(!set.contains(&'\u{03FD}'));
	assert!(!set.contains(&'\0'));

	assert!(!set.insert('\u{03FD}'));

	assert!(set.capacity() >= 0x100);
	assert!(set.len() == 0x2);
	assert!(!set.is_empty());
	assert!(set.contains(&'\u{1F12F}'));
	assert!(set.contains(&'\u{03FD}'));
	assert!(!set.contains(&'\0'));

	assert!(set.remove(&'\u{1F12F}'));

	assert!(set.capacity() >= 0x100);
	assert!(set.len() == 0x1);
	assert!(!set.is_empty());
	assert!(!set.contains(&'\u{1F12F}'));
	assert!(set.contains(&'\u{03FD}'));
	assert!(!set.contains(&'\0'));

	assert!(set.remove(&'\u{03FD}'));

	assert!(set.capacity() >= 0x100);
	assert!(set.len() == 0x0);
	assert!(set.is_empty());
	assert!(!set.contains(&'\u{1F12F}'));
	assert!(!set.contains(&'\u{03FD}'));
	assert!(!set.contains(&'\0'));

	assert!(!set.remove(&'\u{1F12F}'));

	assert!(set.capacity() >= 0x100);
	assert!(set.len() == 0x0);
	assert!(set.is_empty());
	assert!(!set.contains(&'\u{1F12F}'));
	assert!(!set.contains(&'\u{03FD}'));
	assert!(!set.contains(&'\0'));
}

#[test]
fn test_identity_set_from_iter() {
	let data = [
		NonZero::new(0x01).unwrap(),
		NonZero::new(0x1F).unwrap(),
		NonZero::new(0xFF).unwrap(),
		NonZero::new(0x07).unwrap(),
		NonZero::new(0x7F).unwrap(),
		NonZero::new(0x0F).unwrap(),
		NonZero::new(0x03).unwrap(),
		NonZero::new(0x3F).unwrap(),
		//NonZero::new(0x00).unwrap(),
	];

	let set: IdentitySet<NonZero<u8>> = data.into_iter().collect();

	let mut iter = set.into_iter();

	assert_eq!(iter.next().map(NonZero::get), Some(0x01));
	assert_eq!(iter.next().map(NonZero::get), Some(0x03));
	assert_eq!(iter.next().map(NonZero::get), Some(0x07));
	assert_eq!(iter.next().map(NonZero::get), Some(0x0F));
	assert_eq!(iter.next().map(NonZero::get), Some(0x1F));
	assert_eq!(iter.next().map(NonZero::get), Some(0x3F));
	assert_eq!(iter.next().map(NonZero::get), Some(0x7F));
	assert_eq!(iter.next().map(NonZero::get), Some(0xFF));
}
