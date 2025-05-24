**`cenum_utils` provides a minimal set of utilities for querying certain properties of enums in const contexts.**

---

Currently, available features include:

- Accessing the number of variants in an enum.
- Accessing the discriminants for all of an enum's variants.

## Example

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