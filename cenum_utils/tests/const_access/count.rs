use cenum_utils::EnumCount as _;
use crate::common::SimpleEnum;

#[test]
const fn access() {
	let _ = SimpleEnum::COUNT;
}