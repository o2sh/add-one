# add-one

[![crate](https://img.shields.io/badge/crates.io-1.0-orange.svg)](https://crates.io/crates/add-one)
[![documentation](https://img.shields.io/badge/docs-1.0-blue.svg)](https://docs.rs/add-one)

> Returns n + 1.

## Example

```rust
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

```bash
cargo install add-one
add-one 12
13
```
