use cenum_utils::{EnumCount as _, EnumNames as _};
use cenum_utils_derive::ConstEnum;

#[allow(dead_code)]
#[derive(ConstEnum)]
enum Enum {
	X,
	Y,
	Z
}

#[test]
fn count() {
	assert_eq!(Enum::COUNT, 3);
}

#[test]
fn names() {
	assert_eq!(Enum::NAMES, &["X", "Y", "Z"]);
}