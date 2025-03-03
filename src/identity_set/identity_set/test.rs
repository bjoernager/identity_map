// Copyright 2025 Gabriel Bjørnager Jensen.

use crate::identity_set::IdentitySet;

use alloc::vec::Vec;
use bincode::{deserialize_from, serialize_into};
use core::num::NonZero;

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

	assert_eq!(iter.len(), 0x8);

	assert_eq!(iter.next().map(NonZero::get), Some(0x01));
	assert_eq!(iter.next().map(NonZero::get), Some(0x03));
	assert_eq!(iter.next().map(NonZero::get), Some(0x07));
	assert_eq!(iter.next().map(NonZero::get), Some(0x0F));
	assert_eq!(iter.next().map(NonZero::get), Some(0x1F));
	assert_eq!(iter.next().map(NonZero::get), Some(0x3F));
	assert_eq!(iter.next().map(NonZero::get), Some(0x7F));
	assert_eq!(iter.next().map(NonZero::get), Some(0xFF));
	assert_eq!(iter.next().map(NonZero::get), None);
}

#[test]
fn test_identity_set_ops() {
	let set0: IdentitySet<i32> = [
		0x00, 0x01, 0x01, 0x02, 0x03, 0x05,
		0x08, 0x0D, 0x15, 0x22, 0x37, 0x59,
	].into();

	let set1: IdentitySet<i32> = [
		0x02, 0x03, 0x05, 0x07, 0x0B, 0x0D,
		0x11, 0x13, 0x17, 0x1D, 0x1F, 0x25,
	].into();

	let mut iter = set0.intersection(&set1);

	assert_eq!(iter.size_hint(), (0x0, Some(0xB)));

	assert_eq!(iter.next(), Some(&0x02));
	assert_eq!(iter.next(), Some(&0x03));
	assert_eq!(iter.next(), Some(&0x05));
	assert_eq!(iter.next(), Some(&0x0D));
	assert_eq!(iter.next(), None);

	let mut iter = set0.difference(&set1);

	assert_eq!(iter.size_hint(), (0x0, Some(0xB)));

	assert_eq!(iter.next(), Some(&0x00));
	assert_eq!(iter.next(), Some(&0x01));
	assert_eq!(iter.next(), Some(&0x08));
	assert_eq!(iter.next(), Some(&0x15));
	assert_eq!(iter.next(), Some(&0x22));
	assert_eq!(iter.next(), Some(&0x37));
	assert_eq!(iter.next(), Some(&0x59));
	assert_eq!(iter.next(), None);

	let mut iter = set1.difference(&set0);

	assert_eq!(iter.size_hint(), (0x0, Some(0xC)));

	assert_eq!(iter.next(), Some(&0x07));
	assert_eq!(iter.next(), Some(&0x0B));
	assert_eq!(iter.next(), Some(&0x11));
	assert_eq!(iter.next(), Some(&0x13));
	assert_eq!(iter.next(), Some(&0x17));
	assert_eq!(iter.next(), Some(&0x1D));
	assert_eq!(iter.next(), Some(&0x1F));
	assert_eq!(iter.next(), Some(&0x25));
	assert_eq!(iter.next(), None);

	let mut iter = set0.symmetric_difference(&set1);

	assert_eq!(iter.size_hint(), (0x0, Some(0x17)));

	assert_eq!(iter.next(), Some(&0x00));
	assert_eq!(iter.next(), Some(&0x01));
	assert_eq!(iter.next(), Some(&0x07));
	assert_eq!(iter.next(), Some(&0x08));
	assert_eq!(iter.next(), Some(&0x0B));
	assert_eq!(iter.next(), Some(&0x11));
	assert_eq!(iter.next(), Some(&0x13));
	assert_eq!(iter.next(), Some(&0x15));
	assert_eq!(iter.next(), Some(&0x17));
	assert_eq!(iter.next(), Some(&0x1D));
	assert_eq!(iter.next(), Some(&0x1F));
	assert_eq!(iter.next(), Some(&0x22));
	assert_eq!(iter.next(), Some(&0x25));
	assert_eq!(iter.next(), Some(&0x37));
	assert_eq!(iter.next(), Some(&0x59));
	assert_eq!(iter.next(), None);

	let mut iter = set0.union(&set1);

	assert_eq!(iter.size_hint(), (0xB, Some(0x17)));

	assert_eq!(iter.next(), Some(&0x00));
	assert_eq!(iter.next(), Some(&0x01));
	assert_eq!(iter.next(), Some(&0x02));
	assert_eq!(iter.next(), Some(&0x03));
	assert_eq!(iter.next(), Some(&0x05));
	assert_eq!(iter.next(), Some(&0x07));
	assert_eq!(iter.next(), Some(&0x08));
	assert_eq!(iter.next(), Some(&0x0B));
	assert_eq!(iter.next(), Some(&0x0D));
	assert_eq!(iter.next(), Some(&0x11));
	assert_eq!(iter.next(), Some(&0x13));
	assert_eq!(iter.next(), Some(&0x15));
	assert_eq!(iter.next(), Some(&0x17));
	assert_eq!(iter.next(), Some(&0x1D));
	assert_eq!(iter.next(), Some(&0x1F));
	assert_eq!(iter.next(), Some(&0x22));
	assert_eq!(iter.next(), Some(&0x25));
	assert_eq!(iter.next(), Some(&0x37));
	assert_eq!(iter.next(), Some(&0x59));
	assert_eq!(iter.next(), None);
}

#[test]
fn test_identity_set_serialise_deserialise() {
	let input = IdentitySet::<isize>::from([
		i16::MIN as isize,
		0x0,
		i16::MAX as isize,
	]);

	let mut buf = Vec::new();

	serialize_into(&mut buf, &input).unwrap();

	let output: IdentitySet<isize> = deserialize_from(&*buf).unwrap();

	assert_eq!(output, input);
}

#[test]
fn test_identity_set_types() {
	let set0 = IdentitySet::<u8>::from([
		0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7,
		0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF
	]);

	let set1 = IdentitySet::<u8>::from([
		0x0, 0x2, 0x4, 0x6, 0x8, 0xA, 0xC, 0xE,
	]);

	let set2 = IdentitySet::<u8>::from([
		0x1, 0x3, 0x5, 0x7, 0x9, 0xB, 0xD, 0xF,
	]);

	let set3 = IdentitySet::<u8>::new();

	assert!(!set0.is_empty());
	assert!(set0.is_superset(&set0));
	assert!(set0.is_superset(&set1));
	assert!(set0.is_superset(&set2));
	assert!(set0.is_superset(&set3));
	assert!(set0.is_subset(&set0));
	assert!(!set0.is_subset(&set1));
	assert!(!set0.is_subset(&set2));
	assert!(!set0.is_subset(&set3));
	assert!(!set0.is_disjoint(&set0));
	assert!(!set0.is_disjoint(&set1));
	assert!(!set0.is_disjoint(&set2));
	assert!(set0.is_disjoint(&set3));

	assert!(!set1.is_empty());
	assert!(!set1.is_superset(&set0));
	assert!(set1.is_superset(&set1));
	assert!(!set1.is_superset(&set2));
	assert!(set1.is_superset(&set3));
	assert!(set1.is_subset(&set0));
	assert!(set1.is_subset(&set1));
	assert!(!set1.is_subset(&set2));
	assert!(!set1.is_subset(&set3));
	assert!(!set1.is_disjoint(&set0));
	assert!(!set1.is_disjoint(&set1));
	assert!(set1.is_disjoint(&set2));
	assert!(set1.is_disjoint(&set3));

	assert!(!set2.is_empty());
	assert!(!set2.is_superset(&set0));
	assert!(!set2.is_superset(&set1));
	assert!(set2.is_superset(&set2));
	assert!(set2.is_superset(&set3));
	assert!(set2.is_subset(&set0));
	assert!(!set2.is_subset(&set1));
	assert!(set2.is_subset(&set2));
	assert!(!set2.is_subset(&set3));
	assert!(!set2.is_disjoint(&set0));
	assert!(set2.is_disjoint(&set1));
	assert!(!set2.is_disjoint(&set2));
	assert!(set2.is_disjoint(&set3));

	assert!(set3.is_empty());
	assert!(!set3.is_superset(&set0));
	assert!(!set3.is_superset(&set1));
	assert!(!set3.is_superset(&set2));
	assert!(set3.is_superset(&set3));
	assert!(set3.is_subset(&set0));
	assert!(set3.is_subset(&set1));
	assert!(set3.is_subset(&set2));
	assert!(set3.is_subset(&set3));
	assert!(set3.is_disjoint(&set0));
	assert!(set3.is_disjoint(&set1));
	assert!(set3.is_disjoint(&set2));
	assert!(set3.is_disjoint(&set3));
}
