use cenum_utils::{EnumCount, EnumNames, EnumDiscriminants};

#[repr(u8)]
pub enum SimpleEnum {
	X = 0,
	Y = 1,
	Z = 2
}

impl EnumCount for SimpleEnum {
	const COUNT: usize = 3;
}

impl EnumNames for SimpleEnum {
	const NAMES: &[&str] = &["X", "Y", "Z"];
}

impl EnumDiscriminants for SimpleEnum {
	type Discriminant = u8;

	const DISCRIMINANTS: &[Self::Discriminant] = &[0u8, 1u8, 2u8];
}