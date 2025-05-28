**`cenum_utils` provides a minimal set of utilities for querying certain properties of enums in const contexts.**

---

Currently, available features include:

- Accessing the number of variants in an enum.
- Accessing the discriminants for all of an enum's variants.
- Accessing the names for all of an enum's variants.

## Example

```rust
use cenum_utils::*;

#[derive(ConstEnum)]
#[repr(u8)]
enum Enum {
	X,
	Y,
	Z
}

fn test() {
	assert_eq!(Enum::COUNT, 3);
	assert_eq!(Enum::DISCRIMINANTS, &[0, 1, 2]);
	assert_eq!(Enum::NAMES, &["X", "Y", "Z"])
}

const fn const_test() {
	assert!(Enum::COUNT == 3);

	static NAMES: &[u8] = &[b'X', b'Y', b'Z'];

	let mut i = 0;

	while i < Enum::COUNT {
		assert!(Enum::DISCRIMINANTS[i] as usize == i);
		assert!(Enum::NAMES[i].as_bytes()[0] == NAMES[i]);
		i += 1;
	}
}
```