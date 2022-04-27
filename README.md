[![crates.io](https://img.shields.io/crates/v/lazy_fn.svg)](https://crates.io/crates/lazy_fn)
[![docs.rs](https://docs.rs/lazy_fn/badge.svg)](https://docs.rs/lazy_fn/)
[![license](https://img.shields.io/crates/l/lazy_fn)](https://github.com/WilliamVenner/lazy_fn/blob/master/LICENSE)

# `lazy_fn`

[`lazy_static`](https://docs.rs/lazy_static) for functions!

Makes the attributed function "lazy"; it will only be evaluated once and the result will be cached and thus returned as a static reference.

# Usage

In your `Cargo.toml`:

```toml
[dependencies]
lazy_fn = "1"
lazy_static = "1"
```

In your code:

```rust
#[macro_use] extern crate lazy_fn;

#[lazy_fn]
fn maths() -> i32 {
    9 + 10
}

#[lazy_fn]
fn hello_world() -> &'static str {
    "hello, world!"
}

#[lazy_fn]
fn hello_fmt() -> String {
    format!("hello, {}!", "world")
}

let maths: &'static i32 = maths();
let hello_world: &'static str = hello_world();
let hello_fmt: &'static String = hello_fmt();
```