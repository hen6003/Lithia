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

pub mod lisp;
pub mod errors;
pub mod object;
pub mod stdenv;

pub use lisp::Lisp;
