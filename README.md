# rust-luhn2
![Image of Travis CI build status]
(https://travis-ci.org/lunemec/rust-luhn.svg?branch=master)
![Crates.io]
(https://img.shields.io/crates/v/luhn2.svg)

Performs a Luhn algorithm check on given number, returns true/false.

It is `luhn2` because there already is a luhn package but poorly 
documented and with almost no tests (looks like first rust try).

[Documentation](https://lunemec.github.io/rust-luhn/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
luhn2 = "0.1"
```

and this to your crate root:

```rust
extern crate luhn2;
use luhn2::validate;
```
