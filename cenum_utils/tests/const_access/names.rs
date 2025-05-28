use cenum_utils::EnumNames as _;
use crate::common::SimpleEnum;

#[test]
const fn access() {
	let _ = SimpleEnum::NAMES;
}