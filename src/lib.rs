//! # [`lazy_static`](https://docs.rs/lazy_static) for functions!
//!
//! # Usage
//!
//! In your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lazy_fn = "1"
//! lazy_static = "1"
//! ```
//!
//! ```rust
//! # #[macro_use] extern crate lazy_fn;
//! #[lazy_fn]
//! fn maths() -> i32 {
//!     9 + 10
//! }
//!
//! #[lazy_fn]
//! fn hello_world() -> &'static str {
//!     "hello, world!"
//! }
//!
//! #[lazy_fn]
//! fn hello_fmt() -> String {
//!     format!("hello, {}!", "world")
//! }
//!
//! let maths: &'static i32 = maths();
//! let hello_world: &'static str = hello_world();
//! let hello_fmt: &'static String = hello_fmt();
//! ```

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_attribute]
/// Makes the attributed function "lazy"; it will only be evaluated once and the result will be cached and thus returned as a static reference.
///
/// See [`lazy_static`](https://docs.rs/lazy_static) for more information.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate lazy_fn;
/// #[lazy_fn]
/// fn maths() -> i32 {
///     9 + 10
/// }
///
/// #[lazy_fn]
/// fn hello_world() -> &'static str {
///     "hello, world!"
/// }
///
/// #[lazy_fn]
/// fn hello_fmt() -> String {
///     format!("hello, {}!", "world")
/// }
///
/// let maths: &'static i32 = maths();
/// let hello_world: &'static str = hello_world();
/// let hello_fmt: &'static String = hello_fmt();
/// ```
pub fn lazy_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut func = parse_macro_input!(item as syn::ItemFn);

	if func.sig.variadic.is_some() || !func.sig.inputs.is_empty() {
		return r#"compile_error!("a lazy_fn cannot have inputs");"#.parse().unwrap();
	}
	if func.sig.asyncness.is_some() {
		return r#"compile_error!("lazy_fn cannot be async");"#.parse().unwrap();
	}
	if func.sig.constness.is_some() {
		return r#"compile_error!("a const fn doesn't need to be a lazy_fn");"#.parse().unwrap();
	}
	if !func.sig.generics.params.is_empty() || !func.sig.generics.where_clause.as_ref().map(|where_clause| where_clause.predicates.is_empty()).unwrap_or(true) {
		return r#"compile_error!("a lazy_fn cannot have generics");"#.parse().unwrap();
	}

	let block = func.block;
	let output = match &mut func.sig.output {
		syn::ReturnType::Default => syn::parse_quote!(()),
		syn::ReturnType::Type(_, t) => {
			let output = t.clone();
			*t.as_mut() = syn::parse_quote!(&'static #t);
			output
		}
	};

	func.block = syn::parse_quote!({
		::lazy_static::lazy_static!(static ref RESULT: #output = #block;);
		&*RESULT
	});

	func.attrs.insert(0, syn::parse_quote!(#[inline(always)]));

	func.into_token_stream().into()
}