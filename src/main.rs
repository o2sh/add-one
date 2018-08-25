extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt, ParseBigIntError};
use num_traits::FromPrimitive;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let i = &args[1];

    let o = add_one(i);

    match o {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Bad Input: {}", e),
    }
}

fn add_one(number_str: &str) -> Result<BigInt, ParseBigIntError> {
    match number_str.parse::<BigInt>() {
        Ok(n) => Ok(n + 1),
        Err(err) => Err(err),
    }
}

#[test]
fn it_works() {
    assert_eq!(add_one("2").unwrap(),BigInt::from_u64(3).unwrap())
}
