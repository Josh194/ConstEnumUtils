use cenum_utils::{EnumCount as _, EnumDiscriminants as _, EnumNames as _};
use cenum_utils_derive::ConstEnum;

#[derive(ConstEnum)]
#[repr(u8)]
enum Enum {
	X = 0,
	Y = 1,
	Z = 2
}

#[test]
fn count() {
	assert_eq!(Enum::COUNT, 3);
}

#[test]
fn discriminants() {
	assert_eq!(Enum::DISCRIMINANTS, &[0u8, 1u8, 2u8]);
}

#[test]
fn names() {
	assert_eq!(Enum::NAMES, &["X", "Y", "Z"]);
}