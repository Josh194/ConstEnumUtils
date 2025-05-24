use cenum_utils::{EnumCount as _, EnumDiscriminants as _};
use cenum_utils_derive::ConstEnum;

#[derive(ConstEnum)]
#[repr(u8)]
enum Enum {
	X,
	Y,
	Z
}

#[test]
const fn test() {
	assert!(Enum::COUNT == 3);

	let mut i = 0;

	while i < Enum::DISCRIMINANTS.len() {
		assert!(Enum::DISCRIMINANTS[i] as usize == i);
		i += 1;
	}
}