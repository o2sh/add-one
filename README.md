# add-one

[![crate](https://img.shields.io/badge/crates.io-0.2.2-orange.svg)](https://crates.io/crates/add-one)
[![documentation](https://img.shields.io/badge/docs-0.2.2-blue.svg)](https://docs.rs/add-one)
![minimum rustc 1.26](https://img.shields.io/badge/rustc-%2B1.26-red.svg)
[![Travis status](https://travis-ci.org/o2sh/add-one.svg?branch=master)](https://travis-ci.org/o2sh/add-one)


> Returns n + 1.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
add-one = "0.2.2"
```

and this to your crate root:

```rust
extern crate add_one;
```

## Example

```rust
extern crate add_one;
use add_one::add_one;

use std::str;

fn main() {
    let mut bytes = Vec::new();

    match add_one("123".as_bytes(), &mut bytes) {
        Ok(()) => println!("{}", str::from_utf8(&bytes).unwrap()),
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
```

or 

```sh
$ cargo run 12
$ 13
```

## License

Licensed under

 * MIT license ([LICENSE.md](LICENSE.md) or http://opensource.org/licenses/MIT)

## Compatibility

The `add-one` crate is tested for rustc 1.26 and greater.
