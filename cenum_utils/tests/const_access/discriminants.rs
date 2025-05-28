use cenum_utils::EnumDiscriminants as _;
use crate::common::SimpleEnum;

#[test]
const fn access() {
	let _ = SimpleEnum::DISCRIMINANTS;
}