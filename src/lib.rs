/*!
This crate provides a LISP implementation for embedding within rust programs

With support for providing rust functions as LISP functions within the environment

# Usage

This crate is [on crates.io](https://crates.io/crates/regex) and can be
used by adding `lithia` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
lithia = "*"
```

# Example: simple REPL

```rust
let code = "(while t (print (eval (read))))".to_string()

let mut globals = HashMap::new();
let ret = Lisp::new(&mut globals)
    .add_stdenv().unwrap()
    .eval(&code);
```
*/

// No_std require nightly at the moment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(error_in_core))]

extern crate alloc;

pub mod env;
pub mod errors;
pub mod lisp;
pub mod object;

pub use lisp::Lisp;
