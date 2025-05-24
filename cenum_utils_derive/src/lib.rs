/*!
[![github]](https://github.com/Josh194/ConstEnumUtils)&ensp;[![crates-io]](https://crates.io/crates/cenum-utils-derive)&ensp;[![docs-rs]](https://docs.rs/cenum-utils-derive)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

A helper crate providing derive macros for `cenum_utils`.

See the parent docs for more information on use.

# Example

```rust
use cenum_utils_derive::ConstEnum;

#[derive(ConstEnum)]
#[repr(u8)]
enum Enum {
	X,
	Y,
	Z
}
```
*/

use std::{collections::HashSet};

use proc_macro2::{Span, TokenStream};

use quote::quote;
use rustc_hash::{FxBuildHasher};
use syn::{parse_macro_input, DeriveInput, Ident, Type};

/// Derives `EnumCount`, and `EnumDiscriminants` if the enum has a valid primitive `repr` type.
/// 
/// Valid `repr` types are `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, and `isize`.
#[proc_macro_derive(ConstEnum)]
pub fn derive_const_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	impl_const_enum(parse_macro_input!(input as DeriveInput)).unwrap_or_else(|error| error.into_compile_error()).into()
}

thread_local! {
	/// The set of all valid primitive enum representations.
	static PRIMITIVE_TYPES: HashSet<Ident, FxBuildHasher> = {
		let type_tokens = [
			quote! { u8 },
			quote! { u16 },
			quote! { u32 },
			quote! { u64 },
			quote! { u128 },
			quote! { usize },
			quote! { i8 },
			quote! { i16 },
			quote! { i32 },
			quote! { i64 },
			quote! { i128 },
			quote! { isize },
		];

		HashSet::from_iter(type_tokens.into_iter().map(|tokens| syn::parse2(tokens).unwrap()))
	};
}

fn impl_const_enum(item: DeriveInput) -> Result<TokenStream, syn::Error> {
	let syn::Data::Enum(enum_data) = item.data else { return Err(syn::Error::new_spanned(item, "ConstEnum: expected an enum")); };

	// * Get the enum's `repr` type if it exists and is of the expected format.
	let representation: Option<Type> = item.attrs.iter().find_map(|attribute| {
		attribute.meta
			.path()
			.get_ident()
			.and_then(|ident| {
				(ident == "repr")
					.then(|| attribute.parse_args().ok())
					.flatten()
			})
	});

	let enum_name: Ident = item.ident;
	let variant_count: usize = enum_data.variants.len();

	let variant_idents: Box<[&Ident]> = enum_data.variants.iter().map(|variant| &variant.ident).collect();

	// * Get the representation type if it is a valid primitive.
	let enum_discriminant: Option<Type> = representation.and_then(|ty| {
		let Type::Path(type_path) = &ty else { return None; };

		if !type_path.qself.is_none() { return None; };
		let ident: &Ident = type_path.path.get_ident()?;

		PRIMITIVE_TYPES.with(|primitives| primitives.contains(ident)).then_some(ty)
	});

	let crate_name: Ident = Ident::new("cenum_utils", Span::call_site());

	let count_impl: TokenStream = quote! {
		impl ::#crate_name::EnumCount for #enum_name {
			const COUNT: usize = #variant_count;
		}
	};

	let discriminant_impl: Option<TokenStream> = enum_discriminant.map(|discriminant| {
		quote! {
			impl ::#crate_name::EnumDiscriminants for #enum_name {
				type Discriminant = #discriminant;

				const DISCRIMINANTS: &[Self::Discriminant] = &[#(Self::#variant_idents as #discriminant),*];
			}
		}
	});

	Ok(quote! { #count_impl #discriminant_impl })
}