extern crate num_bigint;

use num_bigint::{BigInt, ParseBigIntError};

pub fn add_one(number_str: &str) -> Result<BigInt, ParseBigIntError> {
    number_str.parse::<BigInt>().map(|n| n + 1)
}