#![no_std]

#![feature(allocator_api)]

extern crate alloc;

/// Includes a module and imports it contents.
///
/// The provided visibility denotes the visibility of **all** imported items.
macro_rules! use_mod {
	($vis:vis $name:ident$(,)?) => {
		mod $name;
		$vis use $name::*;
	};
}
pub(crate) use use_mod;

use_mod!(pub identity_map);
use_mod!(pub into_iter);
use_mod!(pub iter);
use_mod!(pub iter_mut);
