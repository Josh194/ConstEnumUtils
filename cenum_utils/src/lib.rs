/*!
[![github]](https://github.com/Josh194/ConstEnumUtils)&ensp;[![crates-io]](https://crates.io/crates/cenum-utils)&ensp;[![docs-rs]](https://docs.rs/cenum-utils)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

`cenum_utils` provides a minimal set of traits and (optionally) derive macros providing const access to certain enum properties.

Currently this includes:
- [EnumCount] — Variant counts.
- [EnumDiscriminants] — Variant discriminants.

Unfortunately, due to rust's currently lack of const trait support, actually interacting with some of the features this crate provides in const contexts can be somewhat difficult.

# Example
```rust
use cenum_utils::{ConstEnum, EnumCount as _, EnumDiscriminants as _};

#[derive(ConstEnum)]
#[repr(u8)]
enum Enum {
	X,
	Y,
	Z
}

const fn test() {
	assert!(Enum::COUNT == 3);

	let mut i = 0;

	while i < Enum::DISCRIMINANTS.len() {
		assert!(Enum::DISCRIMINANTS[i] as usize == i);
		i += 1;
	}
}
```

# Features

- **`derive`** *(enabled by default)* — Derive macros for the core traits provided by this crate.
*/

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use cenum_utils_derive::ConstEnum;

/// A trait providing access to the number of enum variants a type contains.
pub trait EnumCount {
	/// The number of enum variants this type contains.
	const COUNT: usize;
}

/// A trait providing access to the discriminants of an enum's variants.
pub trait EnumDiscriminants {
	type Discriminant: 'static;

	/// A reference to an array containing the discriminants for all enum variants this type contains.
	const DISCRIMINANTS: &[Self::Discriminant];

	/// Returns an iterator over the discriminants for all enum variants this type contains.
	fn discriminants() -> impl Iterator<Item = &'static Self::Discriminant> {
		Self::DISCRIMINANTS.iter()
	}
}

/// A trait providing access to the names of an enum's variants.
pub trait EnumNames {
	/// A reference to an array containing the names for all enum variants this type contains.
	const NAMES: &[&str];

	/// Returns an iterator over the names for all enum variants this type contains.
	fn names() -> impl Iterator<Item = &'static str> {
		Self::NAMES.iter().copied()
	}
}