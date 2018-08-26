# add-one

[![crate](https://img.shields.io/badge/crates.io-0.2-orange.svg)](https://crates.io/crates/add-one)
![minimum rustc 1.26](https://img.shields.io/badge/rustc-%2B1.26-red.svg)
[![Travis status](https://travis-ci.org/02sh/add-one.svg?branch=master)](https://travis-ci.org/02sh/add-one)


> Returns n + 1.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
add-one = "0.2"
```

and this to your crate root:

```rust
extern crate add_one;
```

## Example

```rust
extern crate add_one;
use add_one::add_one;

pub fn add_n(x: &mut usize, n: usize) {
    for _ in 0..usize {
        *x = add_one(x);
    }
}
```

## Compatibility

The `add-one` crate is tested for rustc 1.26 and greater.
